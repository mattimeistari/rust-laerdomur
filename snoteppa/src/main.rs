use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let inntak_vector: Vec<i32> = input.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
    
    let _lengd_vegs: i32 = inntak_vector[0];
    let lengd_breytinga: i32 = inntak_vector[1];
    
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut x_vinstri: Vec<char> = input.trim().chars().collect();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut y_haegri: Vec<char> = input.trim().chars().collect();
    
    
    for _ in 1..=lengd_breytinga {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        //let query_vector: Vec<char> = input.split_whitespace().flat_map(|i| i.chars()).collect();

        let query_vector: Vec<&str> = input.split_whitespace().collect();

        if query_vector[0] == "U" {
            if query_vector[1] == "1" {
                let index: usize = query_vector[2].parse::<usize>().unwrap() - 1;
                //let index: usize = (query_vector[2].to_digit(10).unwrap() - 1) as usize;
                x_vinstri[index] = breyta(x_vinstri[index]);

            } else {
                let index: usize = query_vector[2].parse::<usize>().unwrap() - 1;
                //let index: usize = (query_vector[2].to_digit(10).unwrap() - 1) as usize;
                y_haegri[index] = breyta(y_haegri[index]);
            }
        } else {
            let ans = check(&x_vinstri, &y_haegri);

            if ans {
                println!("Jebb")
            } else {
                println!("Neibb")
            }
        }
    }
}

fn breyta(stafur: char) -> char {
    if stafur == '.' {
        'o'
    } else {
        '.'
    }
}

fn check(top_row: &Vec<char>, bottom_row: &Vec<char>) -> bool {
    for i in 0..top_row.len() {
        if top_row[i] == 'o' {
            let left = if i > 0 { bottom_row[i-1] } else { '.' };
            let mid = bottom_row[i];
            let right = if i + 1 < bottom_row.len() { bottom_row[i+1] } else { '.' };

            if left == 'o' || mid == 'o' || right == 'o' {
                return false;
            }
        }
    }
    true
}