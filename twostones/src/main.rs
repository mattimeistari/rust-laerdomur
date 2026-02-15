use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i32 = input.trim().parse().unwrap();

    if x % 2 == 0 {
        println!("Bob")
    } else {
        println!("Alice")
    }
}