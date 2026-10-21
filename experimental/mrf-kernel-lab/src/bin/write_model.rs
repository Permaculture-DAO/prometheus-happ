use prometheus_mrf_kernel_lab::synthetic_forest;
use std::io::Write;
fn main() {
 let bytes=synthetic_forest().to_bytes().expect("toy model valid");
 let mut f=std::fs::File::create(std::env::args().nth(1).expect("path")).expect("file");
 f.write_all(&bytes).expect("write");
 eprintln!("PMRF_V1_MODEL_BYTES={}",bytes.len());
}