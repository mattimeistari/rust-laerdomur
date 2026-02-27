use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    
    let p: f32 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    
    let mut almennt_sorp: f32 = 0.0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "ekki plast" {
            almennt_sorp += 1.0;
        }
    }

    let plast_hlutfall: f32 = (almennt_sorp / n as f32);

    if p >= plast_hlutfall {
        println!("Jebb")
    } else {
        println!("Neibb")
    }
    
    //I <3 you, Eldur!

}
