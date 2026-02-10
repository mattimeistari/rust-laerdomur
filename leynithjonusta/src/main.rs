use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let mut clean_email = String::new();
    
    for c  in input.chars() {
        if !c.is_whitespace() {
            clean_email.push(c);
        }
    }
    
    println!("{}", clean_email)
}
