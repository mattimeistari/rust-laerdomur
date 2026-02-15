use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i32 = input.trim().parse().unwrap();

    for i in 1..=x {
        println!("{} Abracadabra", i);
    }
}