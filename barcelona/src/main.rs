use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: i32 = input.trim().split_whitespace().last().unwrap().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let index = v.iter().position(|&x| x == n).unwrap();
    
    if index + 1 == 1{
        println!("fyrst");
    } else if index + 1 == 2 {
        println!("naestfyrst");
    } else {
    println!("{} fyrst", index + 1);
    }
}
