use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i128 = input.trim().parse().unwrap();

    let result = if x % 2 == 0 {
        (x / 2) * (x + 1)
    } else {
        x * ((x + 1) / 2)
    };

    println!("{}", result);
}