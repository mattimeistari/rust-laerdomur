use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let x: i32 = input.trim().parse().unwrap();
    let mut e : Vec<i32> =  Vec::new();
    
    for _ in 0..x {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        let value: i32 = line.trim().parse().unwrap();
        e.push(value);
    }
    println!("{}", *e.iter().min().unwrap())
}
