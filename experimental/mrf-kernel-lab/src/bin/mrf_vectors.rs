use prometheus_mrf_kernel_lab::{synthetic_forest, Error};
fn run() -> Result<(), Error> {
    let forest = synthetic_forest();
    let cases: [(i64,i64); 12] = [
        (250,90),(320,120),(300,100),(299,99),(0,0),(-1,-1),
        (i64::MIN,i64::MIN),(i64::MAX,i64::MAX),
        (300,99),(299,100),(300,0),(0,100)
    ];
    for (x0,x1) in cases {
        let y = forest.infer(&[x0,x1])?;
        println!("{x0},{x1},{},{}",y[0],y[1]);
    }
    let mut s: u64 = 0x726567656e657261;
    for _ in 0..4096 {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x0 = (s as i64).wrapping_rem(2001);
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x1 = (s as i64).wrapping_rem(2001);
        let y = forest.infer(&[x0,x1])?;
        println!("{x0},{x1},{},{}",y[0],y[1]);
    }
    Ok(())
}
fn main() { run().expect("frozen toy model must remain valid"); }