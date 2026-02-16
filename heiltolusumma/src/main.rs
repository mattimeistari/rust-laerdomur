use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i128 = input.trim().parse().unwrap();

    println!("{}", x * (x + 1) / 2);
}