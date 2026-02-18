
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i128 = input.trim().parse().unwrap();
    let mut total_list: Vec<i128> = Vec::new();

    for _ in 0..x {
        let mut vec_input = String::new();
        io::stdin().read_line(&mut vec_input).unwrap();
        let num: i128 = vec_input.trim().parse().unwrap();
        total_list.push(num);
    }

    let biggest_number = *total_list.iter().max().unwrap();
    let final_biggest = biggest_number.max(x);

    println!("{}", final_biggest);
}
