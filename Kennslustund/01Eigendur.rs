
fn main() {
    let x = String::from("hello");

    let y = x;

    println!("{x}");
} // error[E0382]

fn main() {
    let x = String::from("hello");

    let y = &x;

    println!("{x}");
} // 15
