use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i32 = input.trim().parse().unwrap();
    let padding: String = " ".to_string();

    // top x
    println!("{}x", padding.repeat((x+1) as usize));

    // top half
    for i in 0..x {
        println!("{}/{} \\",
            padding.repeat((x-i) as usize),
            padding.repeat((i*2) as usize)
        );
    }

    // middle Xs
    println!("x{}x",
    padding.repeat(((x*2)+1) as usize));
    
    // bottom half
    for i in 0..x {
        println!(" {}\\{}/",
            padding.repeat(i as usize),
            padding.repeat((((x-i)*2)-1) as usize)
        );
    }

    // final x
    println!("{}x", padding.repeat((x+1) as usize));

}