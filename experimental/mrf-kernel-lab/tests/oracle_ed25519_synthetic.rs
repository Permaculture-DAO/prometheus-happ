//! Standalone Ed25519 *synthetic* attestation cryptographic test vectors.
//! No production keys, no actual evidence or operator custody proof.
use ed25519_dalek::{Signer, Verifier, SigningKey, Signature, VerifyingKey};
use std::collections::BTreeSet;

fn message(subject:&str,sample:&str,seq:u64,expires:u64,value:i64)->Vec<u8> {
    // Versioned, length-delimited, domain-separated exact payload framing.
    // Production canonical encoding and clock policy require a reviewed ADR.
    let mut out=b"PRM_ORACLE_ATTESTATION_TEST_V1\0".to_vec();
    for x in [subject.as_bytes(),sample.as_bytes()] {
        out.extend_from_slice(&(x.len() as u32).to_be_bytes());
        out.extend_from_slice(x);
    }
    out.extend_from_slice(&seq.to_be_bytes());
    out.extend_from_slice(&expires.to_be_bytes());
    out.extend_from_slice(&value.to_be_bytes());
    out
}
fn verify(key:&VerifyingKey, message:&[u8], signature:&Signature)->bool {
    key.verify(message,signature).is_ok()
}
#[test]
fn valid_signature_and_payload_tampering() {
    let sk=SigningKey::from_bytes(&[7u8;32]);
    let m=message("TEST-site","sample-a",2,1000,17);
    let sig=sk.sign(&m);
    assert!(verify(&sk.verifying_key(),&m,&sig));
    assert!(!verify(&sk.verifying_key(),&message("TEST-site","sample-a",2,1000,18),&sig));
    assert!(!verify(&sk.verifying_key(),&message("TEST-site","sample-a",3,1000,17),&sig));
    assert!(!verify(&sk.verifying_key(),&message("TEST-site","sample-a",2,900,17),&sig));
    assert!(!verify(&SigningKey::from_bytes(&[9u8;32]).verifying_key(),&m,&sig));
}
#[test]
fn canonical_frame_prevents_field_concatenation_ambiguity() {
    assert_ne!(message("ab","c",2,1000,17),message("a","bc",2,1000,17));
}
#[test]
fn replay_guard_rejects_exact_signature_reuse() {
    let sk=SigningKey::from_bytes(&[11u8;32]);
    let msg=message("TEST-site","test-sample",4,5000,9);
    let sig=sk.sign(&msg);
    let key=sk.verifying_key().to_bytes();
    let mut seen=BTreeSet::new();
    assert!(verify(&sk.verifying_key(),&msg,&sig));
    let receipt=(key,msg.clone(),sig.to_bytes());
    assert!(seen.insert(receipt.clone()));
    assert!(!seen.insert(receipt));
}
#[test]
fn expiry_is_policy_not_cryptographic_signature() {
    let sk=SigningKey::from_bytes(&[21u8;32]);
    let m=message("TEST-site","test-sample",5,1000,18);
    let sig=sk.sign(&m);
    assert!(verify(&sk.verifying_key(),&m,&sig)); // signature remains valid after expiry
    let current_policy_time=1001_u64;
    assert!(current_policy_time>1000); // separately enforce freshness in coordinator policy
}
