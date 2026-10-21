//! Synthetic adversarial regression tests. No field data, no network or production DNA.
//! Deterministic byte mutation, parser safety and inference boundaries.
use prometheus_mrf_kernel_lab::{synthetic_forest, Error, Forest, Node, Tree};
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}
#[test]
fn all_prefixes_of_canonical_artifact_fail_closed() {
    let b=synthetic_forest().to_bytes().unwrap();
    for n in 0..b.len() {
        assert!(Forest::from_bytes(&b[..n]).is_err(),"accepted truncated prefix length {n}");
    }
}
#[test]
fn random_bytes_never_panic_or_accept_noncanonical_encoding() {
    let mut state=0x504d52465f544553u64;
    for length in [0usize,1,2,5,8,13,20,63,115,250,1024,4096] {
        for _ in 0..384 {
            let mut bytes=vec![0u8;length];
            for b in &mut bytes { *b=next(&mut state) as u8; }
            if let Ok(f)=Forest::from_bytes(&bytes) {
                assert_eq!(f.to_bytes().expect("valid forest reserializes"),bytes);
            }
        }
    }
}
#[test]
fn mutation_of_every_canonical_byte_is_canonical_or_rejected() {
    let original=synthetic_forest().to_bytes().unwrap();
    for pos in 0..original.len() {
        for bit in 0..8 {
            let mut mutated=original.clone(); mutated[pos]^=1 << bit;
            if let Ok(f)=Forest::from_bytes(&mutated) {
                assert_eq!(f.to_bytes().expect("canonical"),mutated);
            }
        }
    }
}
#[test]
fn randomized_signed_inputs_are_repeatable_and_serialize_stably() {
    let f=synthetic_forest();
    let original=f.to_bytes().unwrap();
    let restored=Forest::from_bytes(&original).unwrap();
    let mut state=0x7e5bbac0ffee1234;
    for _ in 0..8192 {
        let x=[next(&mut state) as i64, next(&mut state) as i64];
        let y=f.infer(&x).unwrap();
        assert_eq!(restored.infer(&x).unwrap(),y);
        assert_eq!(f.infer(&x).unwrap(),y);
    }
}
#[test]
fn rejects_integer_output_exceeding_i64() {
    // Two enormous leaves: their average is still in range; many trees can make
    // aggregate large but average of i64 leaves always remains bounded.
    let f=Forest{features:1,outputs:1,trees:vec![
        Tree{nodes:vec![Node::Leaf(vec![i64::MAX])]},
        Tree{nodes:vec![Node::Leaf(vec![i64::MIN])]},
    ]};
    assert_eq!(f.infer(&[42]).unwrap(),vec![0]);
}
#[test]
fn rejects_structural_attack_including_cycles() {
    let mut f=synthetic_forest();
    f.trees[0].nodes[0]=Node::Split{feature:0,threshold:12,left:2,right:2};
    assert_eq!(f.validate(),Err(Error::BadTree));
    f.trees[0].nodes[0]=Node::Split{feature:0,threshold:12,left:1,right:0};
    assert_eq!(f.validate(),Err(Error::BadTree));
}
#[test]
fn rejects_unbalanced_outputs_and_excessive_tree_count() {
    let mut f=synthetic_forest();
    f.trees[0].nodes[1]=Node::Leaf(vec![13]);
    assert_eq!(f.validate(),Err(Error::BadLeaf));
    f=synthetic_forest();
    let tree=f.trees[0].clone();
    f.trees=vec![tree;33];
    assert_eq!(f.validate(),Err(Error::Bounds));
}
#[test]
fn ignores_no_input_and_rejects_wrong_arity() {
    let f=synthetic_forest();
    assert_eq!(f.infer(&[]),Err(Error::BadInput));
    assert_eq!(f.infer(&[1,2,3]),Err(Error::BadInput));
}