use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let n: i128 = input.trim().parse().unwrap();

    let mut n_hropmerkt: i128 = 1;

    for i in 1..=n {
        n_hropmerkt *= i;
    }

    println!("{}", n_hropmerkt);
}