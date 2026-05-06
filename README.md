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

## Eigendur

### Stack og Heap

Þegar forrit geymir gögn í vinslumynni þá eru tvær leiðir sem öll forrit taka. 
Þessar tvær heita stack og heap.

#### Stack
Stack virkar eins og stafli af diskum. Ef hann lýtur svona út:

<span style="color:red">diskur</span>  
<span style="color:blue">diskur</span>  
<span style="color:green">diskur</span>  
<span style="color:orange">diskur</span>  
<span style="color:purple">diskur</span>  

Þá er voða erfitt að taka appelsínugula diskinn því það að taka hann myndi láta alla hina hrinja.

Þú getur bara sett ofan á efsta disk og tekið efsta disk.
Þetta hugtak heitir last in, first out (LIFO).

En út af því að það er allt að gerast efst í bunkanum þá ertu enga stund að finna og nota diskinn.

Einungis er hægt að setja disk á staflann ef vitað er stærðin á disknum við þýðingartíma (compile time).

Dæmi um staflann er til dæmis þegar kallað er á fall í kerfisforritunarmáli þá eru færibreyturnar sem koma með fallskalli-
```rust
fn samlagning(a: i32, b: i32) -> i32 {
    a + b
}
```
allar geymdar á staflanum þangað til að fallið er búið og öllum diskum eru hentir í ruslið.

#### Heap

Hin leiðin heitir heap og er notuð þegar lokastærð er óþekkt eða getur breyst. Ísl: Kös

Þegar gögn eiga að vera geymd á kösinni fer forritunarmálið að leita af lausum gögnum í vinnsluminninu og pantar sér x mikið pláss.
Þegar gögnin hafa fundið sér sæti í kösinni skilar forritunarmálið bendir (pointer) á stað á vinnsluminninu. Bendirinn fer síðan á staflann.

Síðan ef breytingar gerast við gögnin þá finnur tölvan bendinn og leitar af sætinu sínu skv. leibeiningum bendisins.


Þetta er tiltökulega hægt ferli miðað við að fara beint á staflann út af auka skrefunum sem eiga sér stað við
það að geyma gögn á kösinni.

# Lesson Plan

## 0:00 - 0:10 | Afh Rust?

Flest hraðfær mál eins og C og C++ eru fljót en hjálpa voða lítið við það að halda kóðann sinn öruggann og hagkvæmann.
Til dæmis þegar kóðinn á að hafa tvo vinnsluminnis punkta á sama stað sem veldur minnisspillingu eða hruni.

### Hvað er öðruvísi?
Rust kemur í veg fyrir nánast ef ekki allar svona forritunar galla með því
að þýða kóða þinn ekki nema að allt sé snyrtilega gert hjá forritara.
Þessi þýðandi má vera talinn sem eins konar félaga í tölvunni sem vill einungis
að kóðinn þinn sé vel upp settur. Lýsandi og greind villuboð hjálpa manni að
skrifa kóða án þess að þurfa að hoppa yfir á Google eða gerfigreindina.

### Hverjum er ekki sama?
En þá er ég að pæla hvort þetta skipti einhverju máli? Kannski ekki handa okkur sérstaklega á þessu stigi.
En þetta er helvíti mikilvægt fyrir forritara á NASA. Til dæmis er málið notað til að skrifa
öruggann kóða á gerfihnetti þannig að þeir hrynja ekki útí geim, eða í bakenda á raketu flugs forritum.

### Dæmi
Minecraft Bedrock er að mörgu leyti skrifað í Rust því leikurinn á að geta keyrt á farsímum, leikjatölvum og borðtölvum jafn vel.
Minecraft samfélagið er mikið að skrifa í Rust og fyrir þá sem vita hvað Modrynth er þá er það um 40% Rust.

Discord skipti frá Go yfir í Rust árið 2020 og eftir það varð vinnslutími tífalt hraðari og lagaði stórt vandamál hjá þeim.
Þetta var út af því að rusl gögn voru geymd endalaust og voru að hæga á hjá þeim.

Að lokum sagði Microsoft að 70% af öryggisvandamálum hjá þeim væri út af minnis-vandamálum.

## 0:10 - 0:25 | Smá syntax
Vísa nemendum beint inná [Rust Playground](play.rust-lang.org)


## 0:25 - 0:45 | Hvernig er þetta öruggt?

### Eigendur
Eigendur eru ekkert það flókin. Segjum að ég eigi X. Ég ætla síðan að rétta nemanda hlutnum.
Þá á nemandinn hlutinn og ég á ekkert við hann að gera. (Move) Svona smá eins og kærasta.
En ef að ég tek kannski hlutinn en bara í smá stund og ætla mér síðan að skila honum seinna
þá er það að fá lánað. (Borrow) (&) Svona smá eins og kærasta.

```rust
fn main() { // Búa til breytu og færa owner
    let x = String::from("hello");

    let y = x;

    println!("{x}");
} // error[E0382]
```

```rust
fn main() { // Búa til breytu og borrow
    let x = String::from("hello");

    let y = &x;

    println!("{x}");
} // 15
```

### Breytur
```rust
fn main() {
    let x = String::from("Sæl veröld.");

    println!("{x}");
} // Sæl veröld.
```

Öll forrit verða að vera með main fall og aðal kóðinn fer inn í hann.

```rust
fn main() {
    let x = 10;

    println!("{x}") 
} // 10
```
Þetta er nú bara almen breyta nema það að þú getur ekki breytt henni.

```rust
fn main() {
    let x = 10;

    x += 5;

    println!("{x}");
} // error[E0384]
```
Hérna fáum við villu því ekki má breyta fasta.

Ef þið lesið villuna getið þið lagað kóðann fyrir mig og fengið rétta lausn?

```rust
fn main() {
    let mut x = 10;

    x += 5;

    println!("{x}");
} // 15
```

### For lúpa
```rust
fn main() {
    for i in 0..5 {
        println!("Number: {}", i);
    }
} // 0..=5 <-- er líka hægt og prentar þá líka 5.
``` 

### While lúpur

```rust
fn main() {
    let mut count = 5;

    while count != 0 {
        println!("{count}!");
        count -= 1;
    }

    println!("LIFTOFF!");
}
```

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

### Match
```rust
let number = 3;

match number {
    1 => println!("One!"),
    2 | 3 => println!("Two or Three!"),
    4..=10 => println!("Between four and ten!"),
    _ => println!("Everything else!"),
}
```

### Taka inn inntaki
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

https://docs.google.com/document/d/1AcD4LcwX-zDkaQXT9bLqrroH-RYsjeZyDXhNWs2307Y/edit?tab=t.0

## Að Lokum: Kattis dæmi til að reyna við

- [hipphipp](https://iceland.kattis.com/problems/hipphipp)
- [budarkassi1](https://iceland.kattis.com/problems/budarkassi1)
