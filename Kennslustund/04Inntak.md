# Taka inn inntaki
[leynithjonusta](https://iceland.kattis.com/problems/leynithjonusta)

```rust
use std::io;

fn main() {
    let mut input = String::new(); // Búa til auðann streng til að fylla seinna
    
    io::stdin().read_line(&mut input).unwrap();

    // .read_line(&mut input) les alla línuna sem kemur fram
    //            ^________^ Segir til um hvert línan á að fara sem lesin er.

    // .unwrap() Segir forritinu að það þurfi ekki að tékka hvort það hafi komið eitthvað. Ég veit bara að það hefur gerst.
    
    let mut clean_email = String::new(); // Nýr auður strengur
    
    for c in input.chars() {
        if !c.is_whitespace() {
            clean_email.push(c);
        }
    }
    
    println!("{}", clean_email)
}
```

[storafmaeli](https://iceland.kattis.com/problems/storafmaeli)
```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let number: i32 = input.trim().parse().unwrap();

    if number % 10 == 0 {
        println!("Jebb")
    } else { 
        println!("Neibb")
    }
}
```

## Að Lokum: Kattis dæmi til að reyna við

- [hipphipp](https://iceland.kattis.com/problems/hipphipp)
- [fifa](https://iceland.kattis.com/problems/fifa)
- [budarkassi2](https://iceland.kattis.com/problems/budarkassi2)