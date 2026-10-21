//! GATE 2: TEST-only cryptographic oracle registry policy.
//! Verifies signatures against an explicitly provisioned registry and a
//! reviewer-supplied evaluation time. It cannot verify physical provenance,
//! operator independence, clock trust, calibration or actual custody.
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
struct KeyRecord {
    key: [u8; 32],
    operator: &'static str,
    control_group: &'static str,
    active_from: u64,
    revoked_at: Option<u64>,
}
#[derive(Clone)]
struct Observation {
    key: [u8; 32],
    site: &'static str,
    sample: &'static str,
    method: &'static str,
    data_hash: [u8; 32],
    observed_at: u64,
    expires_at: u64,
    nonce: u64,
    value: i64,
    signature: [u8; 64],
}
#[derive(Debug, PartialEq, Eq)]
enum Decision { CandidateAgreement, Hold(&'static str), Disagreement }

fn frame(o: &Observation) -> Vec<u8> {
    let mut b=b"PRM-ORACLE-V1-TEST\0".to_vec();
    for v in [o.site.as_bytes(), o.sample.as_bytes(), o.method.as_bytes()] {
        b.extend_from_slice(&(v.len() as u32).to_be_bytes());
        b.extend_from_slice(v);
    }
    b.extend_from_slice(&o.data_hash);
    b.extend_from_slice(&o.observed_at.to_be_bytes());
    b.extend_from_slice(&o.expires_at.to_be_bytes());
    b.extend_from_slice(&o.nonce.to_be_bytes());
    b.extend_from_slice(&o.value.to_be_bytes());
    b
}
fn fixture_key(byte: u8, operator: &'static str, control_group: &'static str) -> (SigningKey, KeyRecord) {
    let sk=SigningKey::from_bytes(&[byte; 32]);
    let kr=KeyRecord { key:sk.verifying_key().to_bytes(), operator, control_group,
        active_from:100, revoked_at:None };
    (sk,kr)
}
fn signed(sk:&SigningKey,sample:&'static str,nonce:u64,value:i64) -> Observation {
    let mut o=Observation {
        key:sk.verifying_key().to_bytes(), site:"TEST-site-01",sample,method:"TEST-method-01",
        data_hash:[nonce as u8; 32],observed_at:150,expires_at:300,nonce,value,signature:[0;64]
    };
    o.signature=sk.sign(&frame(&o)).to_bytes();
    o
}
fn fixtures() -> (Vec<KeyRecord>, Vec<Observation>) {
    let (a,ar)=fixture_key(31,"LAB-A","CONTROL-A");
    let (b,br)=fixture_key(32,"LAB-B","CONTROL-B");
    let (c,cr)=fixture_key(33,"LAB-C","CONTROL-C");
    (vec![ar,br,cr],vec![signed(&a,"test-sample-A",1,9),signed(&b,"test-sample-B",2,9),signed(&c,"test-sample-C",3,9)])
}
fn assess(registry:&[KeyRecord], observations:&[Observation], quorum:usize, now:u64) -> Decision {
    if quorum < 2 || now < 100 || observations.len() < quorum {
        return Decision::Hold("insufficient quorum");
    }
    let mut keys=BTreeMap::new();
    for k in registry {
        if k.operator.is_empty() || k.control_group.is_empty() || keys.insert(k.key,k).is_some() {
            return Decision::Hold("invalid identity registry");
        }
    }
    let mut groups=BTreeSet::new();
    let mut samples=BTreeSet::new();
    let mut data_hashes=BTreeSet::new();
    let mut replays=BTreeSet::new();
    let mut values=BTreeSet::new();
    let mut sites=BTreeSet::new();
    let mut methods=BTreeSet::new();
    for o in observations {
        let Some(k)=keys.get(&o.key) else { return Decision::Hold("unknown key"); };
        if o.observed_at < k.active_from || now < o.observed_at || now >= o.expires_at
            || k.revoked_at.is_some_and(|t| now >= t || o.observed_at >= t) {
            return Decision::Hold("untrusted time or revoked key");
        }
        let Ok(pubkey)=VerifyingKey::from_bytes(&o.key) else {
            return Decision::Hold("invalid public key");
        };
        let sig=Signature::from_bytes(&o.signature);
        if pubkey.verify(&frame(o), &sig).is_err() {
            return Decision::Hold("invalid signature");
        }
        if !groups.insert(k.control_group) {
            return Decision::Hold("same controlling operator");
        }
        if !samples.insert(o.sample) || !data_hashes.insert(o.data_hash) {
            return Decision::Hold("duplicated physical-evidence identity");
        }
        if !replays.insert((o.key,o.nonce)) {
            return Decision::Hold("nonce replay within batch");
        }
        sites.insert(o.site);methods.insert(o.method);values.insert(o.value);
    }
    if groups.len() < quorum || sites.len()!=1 || methods.len()!=1 {
        return Decision::Hold("scope/method/quorum mismatch");
    }
    if values.len()>1 { return Decision::Disagreement; }
    Decision::CandidateAgreement
}
#[test] fn three_distinct_keys_are_merely_candidate_agreement() {
    let (r,o)=fixtures();
    assert_eq!(assess(&r,&o,3,200),Decision::CandidateAgreement);
}
#[test] fn one_owner_controls_multiple_keys_is_not_independent() {
    let (mut r,o)=fixtures();
    r[1].control_group="CONTROL-A";
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("same controlling operator"));
}
#[test] fn modelled_signature_forgery_or_tamper_is_held() {
    let (r,mut o)=fixtures();
    o[1].value=80;
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("invalid signature"));
}
#[test] fn swapped_public_key_and_unknown_key_are_held() {
    let (r,mut o)=fixtures();
    o[1].key=SigningKey::from_bytes(&[41u8;32]).verifying_key().to_bytes();
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("unknown key"));
}
#[test] fn expired_or_revoked_key_is_held() {
    let (mut r,o)=fixtures();
    assert_eq!(assess(&r,&o,3,301),Decision::Hold("untrusted time or revoked key"));
    r[0].revoked_at=Some(199);
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("untrusted time or revoked key"));
}
#[test] fn evidence_duplication_is_held() {
    let (r,mut o)=fixtures();
    o[2].sample=o[1].sample;
    // Deliberate, freshly signed duplication must be caught by policy,
    // not only rejected as invalid signature.
    let sk=SigningKey::from_bytes(&[33u8;32]);
    o[2].signature=sk.sign(&frame(&o[2])).to_bytes();
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("duplicated physical-evidence identity"));
}
#[test] fn disagreement_not_overridden_by_threshold() {
    let (r,mut o)=fixtures();
    let c=SigningKey::from_bytes(&[33u8;32]);
    o[2].value=10;
    o[2].signature=c.sign(&frame(&o[2])).to_bytes();
    assert_eq!(assess(&r,&o,3,200),Decision::Disagreement);
}
#[test] fn missing_key_or_quorum_never_becomes_zero() {
    let (mut r,o)=fixtures();r.pop();
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("unknown key"));
    let (r,o)=fixtures();
    assert_eq!(assess(&r,&o[..2],3,200),Decision::Hold("insufficient quorum"));
}
#[test] fn altered_site_scope_invalidates_signature() {
    let (r,mut o)=fixtures();
    o[0].site="OTHER-SITE";
    assert_eq!(assess(&r,&o,3,200),Decision::Hold("invalid signature"));
}
