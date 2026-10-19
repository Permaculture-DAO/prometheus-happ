//! PROMETHEUS MRF kernel laboratory, synthetic-only, not an Holochain zome.
//! PMRF-BIN-v1 experimental encoding. No signatures, model hash, oracle authority
//! or ecological truth claims are implemented here. No production write path.
use std::convert::TryFrom;

pub const MAX_FEATURES: usize = 64;
pub const MAX_OUTPUTS: usize = 16;
pub const MAX_TREES: usize = 32;
pub const MAX_NODES: usize = 1024;
pub const MAX_DEPTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Split { feature: u16, threshold: i64, left: u16, right: u16 },
    Leaf(Vec<i64>),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree { pub nodes: Vec<Node> }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forest {
    pub features: u16,
    pub outputs: u16,
    pub trees: Vec<Tree>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Bounds, InvalidMagic, InvalidVersion, Truncated, TrailingBytes,
    BadTree, BadFeature, BadLeaf, BadInput, Overflow,
}
impl Forest {
    pub fn validate(&self) -> Result<(), Error> {
        let f = self.features as usize;
        let o = self.outputs as usize;
        if f == 0 || f > MAX_FEATURES || o == 0 || o > MAX_OUTPUTS ||
           self.trees.is_empty() || self.trees.len() > MAX_TREES {
            return Err(Error::Bounds);
        }
        for t in &self.trees {
            let n = t.nodes.len();
            if n == 0 || n > MAX_NODES { return Err(Error::Bounds); }
            let mut indegree = vec![0u8; n];
            for (i, node) in t.nodes.iter().enumerate() {
                match node {
                    Node::Leaf(v) if v.len() == o => {},
                    Node::Leaf(_) => return Err(Error::BadLeaf),
                    Node::Split {feature, left, right, ..} => {
                        if *feature as usize >= f { return Err(Error::BadFeature); }
                        let a = *left as usize;
                        let b = *right as usize;
                        if a <= i || b <= i || a >= n || b >= n || a == b {
                            return Err(Error::BadTree);
                        }
                        indegree[a] = indegree[a].checked_add(1).ok_or(Error::BadTree)?;
                        indegree[b] = indegree[b].checked_add(1).ok_or(Error::BadTree)?;
                    },
                }
            }
            if indegree[0] != 0 || indegree[1..].iter().any(|v| *v != 1) {
                return Err(Error::BadTree);
            }
            let mut visited = vec![false; n];
            let mut stack = vec![(0usize, 1usize)];
            while let Some((i, depth)) = stack.pop() {
                if depth > MAX_DEPTH || visited[i] { return Err(Error::BadTree); }
                visited[i] = true;
                if let Node::Split {left, right, ..} = &t.nodes[i] {
                    stack.push((*left as usize, depth + 1));
                    stack.push((*right as usize, depth + 1));
                }
            }
            if visited.iter().any(|v| !*v) { return Err(Error::BadTree); }
        }
        Ok(())
    }
    pub fn infer(&self, x: &[i64]) -> Result<Vec<i64>, Error> {
        self.validate()?;
        if x.len() != self.features as usize { return Err(Error::BadInput); }
        let mut sums = vec![0i128; self.outputs as usize];
        for t in &self.trees {
            let mut i = 0usize;
            loop {
                match &t.nodes[i] {
                    Node::Split {feature, threshold, left, right} => {
                        i = if x[*feature as usize] < *threshold {
                            *left as usize
                        } else { *right as usize };
                    }
                    Node::Leaf(v) => {
                        for (sum, val) in sums.iter_mut().zip(v.iter()) {
                            *sum = sum.checked_add(*val as i128).ok_or(Error::Overflow)?;
                        }
                        break;
                    }
                }
            }
        }
        sums.into_iter()
            .map(|s| i64::try_from(s / (self.trees.len() as i128)).map_err(|_| Error::Overflow))
            .collect()
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        self.validate()?;
        let mut b = Vec::new();
        b.extend_from_slice(b"PMRF\0");
        b.extend_from_slice(&1u16.to_be_bytes());
        b.extend_from_slice(&self.features.to_be_bytes());
        b.extend_from_slice(&self.outputs.to_be_bytes());
        b.extend_from_slice(&(self.trees.len() as u16).to_be_bytes());
        for tree in &self.trees {
            b.extend_from_slice(&(tree.nodes.len() as u16).to_be_bytes());
            for node in &tree.nodes {
                match node {
                    Node::Split {feature, threshold, left, right} => {
                        b.push(0);
                        b.extend_from_slice(&feature.to_be_bytes());
                        b.extend_from_slice(&threshold.to_be_bytes());
                        b.extend_from_slice(&left.to_be_bytes());
                        b.extend_from_slice(&right.to_be_bytes());
                    }
                    Node::Leaf(vals) => {
                        b.push(1);
                        for v in vals { b.extend_from_slice(&v.to_be_bytes()); }
                    }
                }
            }
        }
        Ok(b)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut p = Parser { bytes, pos: 0 };
        if p.take(5)? != b"PMRF\0" { return Err(Error::InvalidMagic); }
        if p.u16()? != 1 { return Err(Error::InvalidVersion); }
        let features = p.u16()?;
        let outputs = p.u16()?;
        let tcount = p.u16()? as usize;
        if features == 0 || features as usize > MAX_FEATURES ||
           outputs == 0 || outputs as usize > MAX_OUTPUTS ||
           tcount == 0 || tcount > MAX_TREES { return Err(Error::Bounds); }
        let mut trees = Vec::with_capacity(tcount);
        for _ in 0..tcount {
            let n = p.u16()? as usize;
            if n == 0 || n > MAX_NODES { return Err(Error::Bounds); }
            let mut nodes = Vec::with_capacity(n);
            for _ in 0..n {
                match p.u8()? {
                    0 => nodes.push(Node::Split {
                        feature: p.u16()?, threshold: p.i64()?,
                        left: p.u16()?, right: p.u16()?,
                    }),
                    1 => {
                        let mut v = Vec::with_capacity(outputs as usize);
                        for _ in 0..outputs { v.push(p.i64()?); }
                        nodes.push(Node::Leaf(v));
                    },
                    _ => return Err(Error::BadTree),
                }
            }
            trees.push(Tree { nodes });
        }
        if p.pos != bytes.len() { return Err(Error::TrailingBytes); }
        let f = Forest { features, outputs, trees };
        f.validate()?;
        Ok(f)
    }
}
struct Parser<'a> { bytes: &'a [u8], pos: usize }
impl<'a> Parser<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(n).ok_or(Error::Bounds)?;
        if end > self.bytes.len() { return Err(Error::Truncated); }
        let res = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(res)
    }
    fn u8(&mut self) -> Result<u8, Error> { Ok(self.take(1)?[0]) }
    fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().map_err(|_| Error::Truncated)?))
    }
    fn i64(&mut self) -> Result<i64, Error> {
        Ok(i64::from_be_bytes(self.take(8)?.try_into().map_err(|_| Error::Truncated)?))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn toy() -> Forest {
        Forest { features: 2, outputs: 2, trees: vec![
            Tree { nodes: vec![
                Node::Split {feature: 0, threshold: 300, left: 1, right: 2},
                Node::Leaf(vec![15, -5]),
                Node::Leaf(vec![4, 2]),
            ] },
            Tree { nodes: vec![
                Node::Split {feature: 1, threshold: 100, left: 1, right: 2},
                Node::Leaf(vec![1, 1]),
                Node::Leaf(vec![3, -2]),
            ] },
        ] }
    }
    #[test] fn golden_left() { assert_eq!(toy().infer(&[250,90]), Ok(vec![8,-2])); }
    #[test] fn golden_right() { assert_eq!(toy().infer(&[320,120]), Ok(vec![3,0])); }
    #[test] fn equal_threshold_goes_right() {
        assert_eq!(toy().infer(&[300,100]), Ok(vec![3,0]));
    }
    #[test] fn bytes_roundtrip() {
        let f = toy();
        let bytes = f.to_bytes().unwrap();
        assert_eq!(Forest::from_bytes(&bytes), Ok(f));
    }
    #[test] fn trailing_bytes_rejected() {
        let mut bytes = toy().to_bytes().unwrap(); bytes.push(0);
        assert_eq!(Forest::from_bytes(&bytes), Err(Error::TrailingBytes));
    }
    #[test] fn truncation_rejected() {
        let mut bytes = toy().to_bytes().unwrap(); bytes.pop();
        assert_eq!(Forest::from_bytes(&bytes), Err(Error::Truncated));
    }
    #[test] fn unrecognized_tag_rejected() {
        let mut bytes = toy().to_bytes().unwrap();
        bytes[15] = 7;
        assert_eq!(Forest::from_bytes(&bytes), Err(Error::BadTree));
    }
    #[test] fn bad_index_rejected() {
        let mut f = toy();
        f.trees[0].nodes[0] = Node::Split{feature:0,threshold:300,left:0,right:2};
        assert_eq!(f.validate(), Err(Error::BadTree));
    }
    #[test] fn out_of_range_feature_rejected() {
        let mut f = toy();
        f.trees[0].nodes[0] = Node::Split{feature:9,threshold:300,left:1,right:2};
        assert_eq!(f.validate(), Err(Error::BadFeature));
    }
    #[test] fn wrong_feature_vector_rejected() {
        assert_eq!(toy().infer(&[10]), Err(Error::BadInput));
    }
    #[test] fn negative_fraction_rounds_toward_zero() {
        let f = Forest {features:1,outputs:1,trees:vec![
          Tree{nodes:vec![Node::Leaf(vec![-1])]},
          Tree{nodes:vec![Node::Leaf(vec![0])]},
        ]};
        assert_eq!(f.infer(&[0]), Ok(vec![0]));
    }
    #[test] fn integrity_no_orphan_node() {
        let mut f = toy(); f.trees[0].nodes.push(Node::Leaf(vec![7,8]));
        assert_eq!(f.validate(), Err(Error::BadTree));
    }
}


/// Test-only synthetic forest; no trained scientific parameters or evidence.
pub fn synthetic_forest() -> Forest {
    Forest { features: 2, outputs: 2, trees: vec![
        Tree { nodes: vec![
            Node::Split { feature: 0, threshold: 300, left: 1, right: 2 },
            Node::Leaf(vec![15, -5]), Node::Leaf(vec![4, 2]),
        ] },
        Tree { nodes: vec![
            Node::Split { feature: 1, threshold: 100, left: 1, right: 2 },
            Node::Leaf(vec![1, 1]), Node::Leaf(vec![3, -2]),
        ] },
    ] }
}

/// Synthetic ABI smoke-test export. This is not an HDI validation callback.
#[no_mangle]
pub extern "C" fn prometheus_mrf_lab_y0(f0: i64, f1: i64) -> i64 {
    synthetic_forest().infer(&[f0, f1]).expect("fixed toy model")[0]
}
/// Synthetic ABI smoke-test export. This is not a scientific prediction.
#[no_mangle]
pub extern "C" fn prometheus_mrf_lab_y1(f0: i64, f1: i64) -> i64 {
    synthetic_forest().infer(&[f0, f1]).expect("fixed toy model")[1]
}
#[cfg(test)]
mod export_tests {
    use super::*;
    #[test]
    fn native_abi_smoke() {
        assert_eq!(prometheus_mrf_lab_y0(250, 90), 8);
        assert_eq!(prometheus_mrf_lab_y1(250, 90), -2);
        assert_eq!(prometheus_mrf_lab_y0(320, 120), 3);
        assert_eq!(prometheus_mrf_lab_y1(320, 120), 0);
    }
}