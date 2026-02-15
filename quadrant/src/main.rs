use std::io;

fn main() {
    let mut x_input: String= String::new();
    let mut y_input: String= String::new();
    io::stdin().read_line(&mut x_input).unwrap();
    io::stdin().read_line(&mut y_input).unwrap();

    let x: i32 = x_input.trim().parse().unwrap();
    let y: i32 = y_input.trim().parse().unwrap();

    match (x > 0, y > 0) {
        (true, true) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
        (true, false) => println!("4"),
    }
}