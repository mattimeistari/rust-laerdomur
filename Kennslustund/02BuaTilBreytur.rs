fn main() {
    let x = String::from("Sæl veröld.");

    println!("{x}");
} // Sæl veröld.


// Öll forrit verða að vera með main fall og aðal kóðinn fer inn í hann.

fn main() {
    let x = 10;

    println!("{x}") 
} // 10

// Þetta er nú bara almen breyta nema það að þú getur ekki breytt henni.

fn main() {
    let x = 10;

    x += 5;

    println!("{x}");
} // error[E0384]

// Hérna fáum við villu því ekki má breyta fasta.

// Ef þið lesið villuna getið þið lagað kóðann fyrir mig og fengið rétta lausn?