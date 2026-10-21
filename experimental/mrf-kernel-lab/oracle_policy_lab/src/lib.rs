//! TEST-only oracle attestations, not global consensus, ecological proof, or a claim verifier.
//! Independent operators/methods are determined from a pre-approved *trusted* registry.
//! Every invalid signed claim forces review, never a partial k-of-n false PASS.
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::collections::{BTreeMap, BTreeSet};

const DOMAIN: &[u8] = b"PROMETHEUS/ORACLE-ATTESTATION/TEST/V1\0";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedAuthority {
    pub operator_group: String,
    pub device_id: String,
    pub method_id: String,
}
#[derive(Clone, Debug)]
pub struct Policy {
    pub registry: BTreeMap<[u8;32], TrustedAuthority>,
    pub revoked_keys: BTreeSet<[u8;32]>,
    pub min_groups: usize,
    pub min_methods: usize,
    pub max_age_secs: u64,
}
#[derive(Clone, Debug)]
pub struct Attestation {
    pub signer: [u8;32],
    pub batch_id: [u8;32],
    pub distinct_sample_hash: [u8;32],
    pub observed_at: u64,
    pub calibration_valid: bool,
    pub supports_hypothesis: bool,
    pub signature: [u8;64],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome { Agreement, Disagreement, Unknown, RequiresReview }

// Canonical signed payload: domain separated, fixed-size, no sorting ambiguity.
pub fn signing_bytes(a: &Attestation) -> Vec<u8> {
    let mut out = Vec::with_capacity(DOMAIN.len()+32+32+8+2);
    out.extend_from_slice(DOMAIN);
    out.extend_from_slice(&a.batch_id);
    out.extend_from_slice(&a.distinct_sample_hash);
    out.extend_from_slice(&a.observed_at.to_be_bytes());
    out.push(u8::from(a.calibration_valid));
    out.push(u8::from(a.supports_hypothesis));
    out
}

// No network, no wall-clock read, no runtime DHT lookups. Caller freezes 'now'.
pub fn evaluate(policy: &Policy, attestations: &[Attestation], now: u64) -> Outcome {
    if policy.min_groups < 2 || policy.min_methods < 2 || policy.max_age_secs == 0 {
        return Outcome::RequiresReview;
    }
    if attestations.is_empty() { return Outcome::Unknown; }
    let batch = attestations[0].batch_id;
    let mut signers = BTreeSet::new();
    let mut samples = BTreeSet::new();
    let mut groups_yes = BTreeSet::new();
    let mut groups_no = BTreeSet::new();
    let mut methods_yes = BTreeSet::new();
    let mut methods_no = BTreeSet::new();
    let mut devices = BTreeSet::new();
    for a in attestations {
        if a.batch_id != batch || !a.calibration_valid ||
            policy.revoked_keys.contains(&a.signer) ||
            !signers.insert(a.signer) || !samples.insert(a.distinct_sample_hash) {
            return Outcome::RequiresReview;
        }
        let Some(authority) = policy.registry.get(&a.signer) else {
            return Outcome::RequiresReview;
        };
        if authority.operator_group.is_empty() || authority.device_id.is_empty() ||
           authority.method_id.is_empty() || !devices.insert(&authority.device_id) {
            // Two signatures from the same physical device are not independent.
            return Outcome::RequiresReview;
        }
        let Some(age) = now.checked_sub(a.observed_at) else {
            return Outcome::RequiresReview;
        };
        if age > policy.max_age_secs { return Outcome::RequiresReview; }
        let Ok(key) = VerifyingKey::from_bytes(&a.signer) else {
            return Outcome::RequiresReview;
        };
        let signature = Signature::from_bytes(&a.signature);
        if key.verify(&signing_bytes(a), &signature).is_err() {
            return Outcome::RequiresReview;
        }
        if a.supports_hypothesis {
            groups_yes.insert(&authority.operator_group);
            methods_yes.insert(&authority.method_id);
        } else {
            groups_no.insert(&authority.operator_group);
            methods_no.insert(&authority.method_id);
        }
    }
    if !groups_yes.is_empty() && !groups_no.is_empty() { return Outcome::Disagreement; }
    if groups_yes.len() >= policy.min_groups && methods_yes.len() >= policy.min_methods {
        // Only cryptographically verified signer agreement, NOT environmental truth.
        Outcome::Agreement
    } else {
        // Includes less than threshold or all-negative (no ecological endorsement).
        Outcome::Unknown
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EcologicalSnapshot {
    pub soil: Option<i64>,
    pub water: Option<i64>,
    pub biodiversity: Option<i64>,
    pub external_inputs: Option<i64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SentinelOutcome { CandidateOnly, Unknown, RequiresReview }
// No aggregation of gains to offset worsened sentinel metrics; no financial outputs.
pub fn ecological_sentinel(base: &EcologicalSnapshot, observed: &EcologicalSnapshot) -> SentinelOutcome {
    let (
        Some(bs),Some(bw),Some(bb),Some(bi),Some(os),Some(ow),Some(ob),Some(oi)
    ) = (base.soil,base.water,base.biodiversity,base.external_inputs,
         observed.soil,observed.water,observed.biodiversity,observed.external_inputs)
    else { return SentinelOutcome::Unknown };
    if os < bs || ow < bw || ob < bb || oi > bi {
        return SentinelOutcome::RequiresReview;
    }
    SentinelOutcome::CandidateOnly
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use sha2::{Digest, Sha256};

    fn h(s: &str)->[u8;32] { Sha256::digest(s.as_bytes()).into() }
    fn identity(seed: u8)->SigningKey { SigningKey::from_bytes(&[seed;32]) }
    fn attestation(key:&SigningKey, sample:&str, yes:bool)->Attestation {
        let mut a=Attestation {
            signer:key.verifying_key().to_bytes(),
            batch_id:h("TEST-site-batch-1"), distinct_sample_hash:h(sample),
            observed_at:1000, calibration_valid:true,
            supports_hypothesis:yes, signature:[0u8;64],
        };
        a.signature=key.sign(&signing_bytes(&a)).to_bytes();
        a
    }
    fn setup()->(Policy,SigningKey,SigningKey,SigningKey) {
        let a=identity(1); let b=identity(2); let c=identity(3);
        let mut registry=BTreeMap::new();
        for (key,group,device,method) in [
            (&a,"operator-A","sensor-1","method-X"),
            (&b,"operator-B","sensor-2","method-Y"),
            (&c,"operator-A","sensor-3","method-Z")
        ] {
            registry.insert(key.verifying_key().to_bytes(),TrustedAuthority {
                operator_group:group.into(),device_id:device.into(),method_id:method.into()
            });
        }
        (Policy{registry,revoked_keys:BTreeSet::new(),min_groups:2,min_methods:2,max_age_secs:60},a,b,c)
    }
    #[test] fn independent_signed_agreement_is_only_agreement() {
        let (p,a,b,_)=setup();
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&b,"B",true)],1020),Outcome::Agreement);
    }
    #[test] fn distinct_signatures_same_controlled_group_are_not_independent() {
        let (p,a,_,c)=setup();
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&c,"C",true)],1020),Outcome::Unknown);
    }
    #[test] fn forged_signature_rejected() {
        let (p,a,b,_)=setup(); let mut x=attestation(&b,"B",true);
        x.signature[2]^=1;
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),x],1020),Outcome::RequiresReview);
    }
    #[test] fn repeated_physical_sample_rejected() {
        let (p,a,b,_)=setup();
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&b,"A",true)],1020),Outcome::RequiresReview);
    }
    #[test] fn expired_or_future_attestation_rejected() {
        let (p,a,b,_)=setup(); let v=[attestation(&a,"A",true),attestation(&b,"B",true)];
        assert_eq!(evaluate(&p,&v,1061),Outcome::RequiresReview);
        assert_eq!(evaluate(&p,&v,999),Outcome::RequiresReview);
    }
    #[test] fn revoked_signer_rejected() {
        let (mut p,a,b,_)=setup(); p.revoked_keys.insert(a.verifying_key().to_bytes());
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&b,"B",true)],1020),Outcome::RequiresReview);
    }
    #[test] fn missing_calibration_rejected() {
        let (p,a,b,_)=setup(); let mut v=attestation(&b,"B",true);
        v.calibration_valid=false;
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),v],1020),Outcome::RequiresReview);
    }
    #[test] fn disagreeing_independent_signers_return_disagreement() {
        let (p,a,b,_)=setup();
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&b,"B",false)],1020),Outcome::Disagreement);
    }
    #[test] fn altered_claim_after_signing_is_rejected() {
        let (p,a,b,_)=setup(); let mut v=attestation(&b,"B",false);
        v.supports_hypothesis=true;
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),v],1020),Outcome::RequiresReview);
    }
    #[test] fn unknown_signer_rejected() {
        let (p,a,b,_)=setup();let rogue=identity(12);
        assert_eq!(evaluate(&p,&[attestation(&a,"A",true),attestation(&b,"B",true),attestation(&rogue,"Z",true)],1020),Outcome::RequiresReview);
    }
    #[test] fn ecological_worsening_never_offset_by_another_gain() {
        let b=EcologicalSnapshot{soil:Some(10),water:Some(10),biodiversity:Some(10),external_inputs:Some(10)};
        let improved=EcologicalSnapshot{soil:Some(1000),water:Some(9),biodiversity:Some(13),external_inputs:Some(3)};
        assert_eq!(ecological_sentinel(&b,&improved),SentinelOutcome::RequiresReview);
    }
    #[test] fn missing_ecology_is_not_zero_and_not_positive() {
        let b=EcologicalSnapshot{soil:None,water:Some(10),biodiversity:Some(10),external_inputs:Some(10)};
        assert_eq!(ecological_sentinel(&b,&b),SentinelOutcome::Unknown);
    }
    #[test] fn no_regression_only_produces_candidate_not_certification() {
        let b=EcologicalSnapshot{soil:Some(10),water:Some(10),biodiversity:Some(10),external_inputs:Some(10)};
        assert_eq!(ecological_sentinel(&b,&b),SentinelOutcome::CandidateOnly);
    }
}
