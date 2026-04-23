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

