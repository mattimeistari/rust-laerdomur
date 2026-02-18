use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let x: i32 = input.trim().parse().unwrap();
    let mut top_number: i32 = i32::MIN;
    
    let mut top_string: String = String::new();
    
    
    for _ in 0..x {
        let mut new_string = String::new();
        io::stdin().read_line(&mut new_string).unwrap();
        
        let running_int = new_string
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        
        if running_int > top_number {
            top_string = new_string.split_whitespace().next().unwrap().to_string();
            top_number = running_int
        }
    }
    
    println!("{}", top_string)
}
