use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let my_string = input.trim().to_string();
    let string_vec: Vec<char> = input
        .trim()
        .chars()
        .collect();

    println!("my string: {}", my_string);
    println!("{:?}", string_vec);
}
