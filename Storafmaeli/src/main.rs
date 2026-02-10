use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let number: i32 = input.trim().parse().unwrap();

    if number % 10 == 0 {
        println!("Jebb")
    } else { 
        println!("Neibb")
    }
}
