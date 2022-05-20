use std::time::Instant;

static N:u64 = 1_000_000_000;

fn main() {
    let start = Instant::now();
    let pi:f64 = calculatepi(N);
    let duration = start.elapsed();
    println!("π : {} \nΔt : {} s", 
        pi.to_string(), 
        duration.as_secs_f64());

}

fn calculatepi(n: u64) -> f64 {
    let numerator:f64           = 4.0;
    let mut denominator:f64     = 1.0;
    let mut operation:f64       = 1.0;
    let mut pi:f64              = 0.0;

    for _ in 1..n {
        pi              = pi + operation * (numerator / denominator);
        denominator     += 2.0;
        operation       *= -1.0;
    }
    return pi;
}