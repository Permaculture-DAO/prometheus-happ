//! Synthetic verified-key oracle registry; offline deterministic policy test.
//! Not a trusted live operator registry or ecological evidence.
use ed25519_dalek::{Signer, Signature, SigningKey, Verifier, VerifyingKey};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
struct Identity { operator:u16, key:[u8;32], revoked_at:Option<u64> }
#[derive(Clone)]
struct Claim { operator:u16, key:[u8;32], sample:u64, seq:u64, issued:u64, expires:u64,
    value:i64, sig:[u8;64] }
#[derive(Debug,PartialEq,Eq)]
enum Decision { CandidateAgreement, Unknown, Disagreement, Reject }

fn frame(c:&Claim)->Vec<u8> {
    let mut b=b"PROMETHEUS_ORACLE_TEST_ONLY_V1\0".to_vec();
    b.extend_from_slice(&c.operator.to_be_bytes());
    b.extend_from_slice(&c.key);
    b.extend_from_slice(&c.sample.to_be_bytes());
    b.extend_from_slice(&c.seq.to_be_bytes());
    b.extend_from_slice(&c.issued.to_be_bytes());
    b.extend_from_slice(&c.expires.to_be_bytes());
    b.extend_from_slice(&c.value.to_be_bytes());
    b
}
fn sign(mut c:Claim,sk:&SigningKey)->Claim { c.sig=sk.sign(&frame(&c)).to_bytes();c }
fn check(records:&[Claim],registry:&BTreeMap<[u8;32],Identity>,
    now:u64,required:usize)->Decision {
    if required<2 || records.len()<required {return Decision::Unknown;}
    let mut owners=BTreeSet::new();
    let mut samples=BTreeSet::new();
    let mut replay=BTreeSet::new();
    let mut values=BTreeSet::new();
    for c in records {
        let Some(id)=registry.get(&c.key) else {return Decision::Reject};
        if id.operator!=c.operator || id.key!=c.key {return Decision::Reject;}
        if id.revoked_at.is_some_and(|t| c.issued>=t || now>=t) {return Decision::Reject;}
        if c.issued>now || c.expires<now || c.expires<c.issued {return Decision::Reject;}
        let Ok(vk)=VerifyingKey::from_bytes(&c.key) else {return Decision::Reject};
        if vk.verify(&frame(c),&Signature::from_bytes(&c.sig)).is_err() {return Decision::Reject;}
        if !owners.insert(c.operator) || !samples.insert(c.sample) ||
           !replay.insert((c.key,c.seq)) {return Decision::Reject;}
        values.insert(c.value);
    }
    if owners.len()<required {return Decision::Unknown;}
    if values.len()!=1 {return Decision::Disagreement;}
    Decision::CandidateAgreement
}
fn fixtures()->(Vec<Claim>,BTreeMap<[u8;32],Identity>) {
    let mut claims=Vec::new();let mut registry=BTreeMap::new();
    for (index,seed) in [11u8,12,13].into_iter().enumerate() {
        let sk=SigningKey::from_bytes(&[seed;32]);
        let key=sk.verifying_key().to_bytes();
        registry.insert(key,Identity{operator:index as u16,key,revoked_at:None});
        let c=Claim{operator:index as u16,key,sample:100+index as u64,seq:1,
             issued:10,expires:30,value:7,sig:[0;64]};
        claims.push(sign(c,&sk));
    }
    (claims,registry)
}
#[test] fn three_signed_distinct_operators_candidate_not_certification(){
 let(c,r)=fixtures();assert_eq!(check(&c,&r,20,3),Decision::CandidateAgreement);
}
#[test] fn single_operator_multiple_keys_not_independence(){
 let(mut c,mut r)=fixtures();
 // second registered key is under first operator's corporate control
 let key=c[1].key;r.get_mut(&key).unwrap().operator=0;
 c[1].operator=0;c[1]=sign(c[1].clone(),&SigningKey::from_bytes(&[12;32]));
 assert_eq!(check(&c,&r,20,3),Decision::Reject);
}
#[test] fn tampered_attestation_is_rejected(){
 let(mut c,r)=fixtures();c[0].value=77;assert_eq!(check(&c,&r,20,3),Decision::Reject);
}
#[test] fn revoked_key_rejected(){
 let(c,mut r)=fixtures();r.get_mut(&c[0].key).unwrap().revoked_at=Some(15);
 assert_eq!(check(&c,&r,20,3),Decision::Reject);
}
#[test] fn stale_and_future_dated_rejected(){
 let(c,r)=fixtures();
 assert_eq!(check(&c,&r,31,3),Decision::Reject);
 assert_eq!(check(&c,&r,9,3),Decision::Reject);
}
#[test] fn replayed_sample_rejected(){
 let(mut c,r)=fixtures();
 c[1].sample=c[0].sample;
 c[1]=sign(c[1].clone(),&SigningKey::from_bytes(&[12;32]));
 assert_eq!(check(&c,&r,20,3),Decision::Reject);
}
#[test] fn divergent_signed_readings_flag_disagreement(){
 let(mut c,r)=fixtures();c[1].value=8;
 c[1]=sign(c[1].clone(),&SigningKey::from_bytes(&[12;32]));
 assert_eq!(check(&c,&r,20,3),Decision::Disagreement);
}
#[test] fn unknown_signer_rejected(){
 let(c,mut r)=fixtures();r.remove(&c[0].key);
 assert_eq!(check(&c,&r,20,3),Decision::Reject);
}
