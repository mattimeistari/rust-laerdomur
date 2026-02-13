use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();
        
        let s: String = input
            .trim()
            .chars()
            .filter(|c| c.is_uppercase())
            .collect();
            
    println!("{}", s);
}
