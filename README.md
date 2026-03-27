# Hérna kemur plan:

# Glósur

## Mut er ekki the only way.
Segjum að ég sé með
```rust
let x: i8 = 67;

let x = x - 25;

{
    let x = x*2;
    println!("x sinnum 2 = {}", x);
    // Prentar 84
}

println!("hreint x = {}", x);
// Prentar 42
```
Hægt er líka að breyta týpu:
```rust
let spaces: &str = "   ";
let spaces: usize = spaces.len();
    
println!("{}", spaces)
```
```rust
let mut spaces: &str = "   ";
let spaces: usize = spaces.len();
    
println!("{}", spaces)
```
Þetta, hindsvegar, myndi ekki virka því að "spaces" vill núna bara breytast í aðra &str og ekki neitt annað.

## isize vs usize
Þegar rust geymir tölu þá velur þú annaðhvort heiltölur eða ræðar tölur. Eftir það getur þú valið á milli náttúrulegum eða neikvæðum tölum, annars velur málið fyrir þig. Þú velur sjálfur hvað á að nota til að spara gögnum sem geta verið notuð annars staðar.

Þegar skilgreind er náttúruleg tala, notar maður u8, u16, u64, u128.
Þessar mismunandi tölur tákna veldi af tveimur: u8 = 0 .. =2^8.

```rust
    let decimal: u8 = 2_5_5;     // 255
    let hex: u8 = 0xff;          // 255
    let octal: u8 = 0o377;       // 255
    let binary: u8 = 0b11111111; // 255
    let byte: u8 = b'\xff'; // ÿ // 255
```


Ef þér langar að hafa neikvæðar tölur í forritinu þínu velur þú datatýpu sem er svipuð en byrjar á "i". I táknar integer (heiltölur) og u táknar unsigned integer (náttúrulegar tölur).
i tekur in jafn margar tölur og u, nema dreift jafnt á milli + og - ásna.  −(2^7) til (2^7) − 1.
þ.e −128 til 127.

Afhverju þarf ég að velja ef tölvan ræður fyrir mig?

## Expressions vs Statements

Statement er grein af kóða sem skilar engu frá sér. 
```rust
let y = 6;
```

Þetta segir bara hvað y er og skilar engu frá sér.

Hindsvegar er segð (expression) eitthvað sem skilar frá sér gildi
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Í þessum kóða er notað gildissvið (scope) {} sem skilar alltaf eitthverju frá sér eftir keyrslu.
Líkt og () í stærðfræði þar sem unnið er úr sviganum og síðan kemur lokasvar.
5 * (6 + 7) ==== 5 * (13) ==== 5 * 13 (expression búin að skila sér) ==== 65
Expressions eru líka kall í fall eða fjölvaskipun (macro)