use std::io;

fn main() {
    let mut dist_input = String::new();
    let mut bounces_input = String::new();

    io::stdin().read_line(&mut dist_input).unwrap();
    io::stdin().read_line(&mut bounces_input).unwrap();

    let dist: f64 = dist_input.trim().parse().unwrap();
    let bounces: u32 = bounces_input.trim().parse().unwrap();
    let mut dist_final: f64 = dist;

    for i in 1..(bounces + 1) {
        // dist * 0.5 ^ i
        dist_final += dist * 0.5_f64.powf(i as f64);
    }

    println!("{}", dist_final);
}
