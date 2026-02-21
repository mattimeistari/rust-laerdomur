use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: i32 = input.trim().parse().unwrap();
    
    let mut shi: Vec<i32> = Vec::new();
    
    for _ in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        
        shi.push(num.trim().parse().unwrap());
    }
    
    for value in shi.iter().rev() {
        println!("{}", value)
    }
}
