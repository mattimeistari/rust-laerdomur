use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: i32 = input.trim().parse().unwrap();
    
    if n == 2 {
        println!("28")
    } else if n % 2 == 0 && n <= 7 {
        println!("30")
    } else if n % 2 != 0 && n >= 8 {
        println!("30")
    } else {
        println!("31")
    }
}
