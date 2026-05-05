// For lúpa

fn main() {
    for i in 0..5 {
        println!("Number: {}", i);
    }
} // 0..=5 <-- er líka hægt og prentar þá líka 5.


// While lúpur


fn main() {
    let mut count = 5;

    while count != 0 {
        println!("{count}!");
        count -= 1;
    }

    println!("LIFTOFF!");
}

fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}



// Match
let number = 3;

match number {
    1 => println!("One!"),
    2 | 3 => println!("Two or Three!"),
    4..=10 => println!("Between four and ten!"),
    _ => println!("Everything else!"),
}