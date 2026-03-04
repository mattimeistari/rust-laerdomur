use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();

    let mut n: Vec<i128> = input
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    let mut iteration_count = 0;

    while !n.is_empty() {
        let mut current_max: i128 = i128::MIN;
        let mut next_gen: Vec<i128> = Vec::new();

        for &i in &n {
            if i > current_max {
                current_max = i;
            } else {
                next_gen.push(i);
            }
        }
        n = next_gen;
        iteration_count += 1;
    }

    println!("{}", iteration_count);
}

// bara 50 stig :(