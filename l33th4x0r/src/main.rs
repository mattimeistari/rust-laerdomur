use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let is_monospace = input.trim() == "monospace";
    
    let mut terminal_bg_color = [0i32; 3];
    for i in 0..3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        terminal_bg_color[i] = input.trim().parse().unwrap();
    }
    
    let mut font_color = [0i32; 3];
    for i in 0..3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        font_color[i] = input.trim().parse().unwrap();
    }
    
    let mut count = 0;
    
    if terminal_bg_color[0].abs()
        + terminal_bg_color[1].abs()
        + terminal_bg_color[2].abs()
        <= 25
    {
        count += 1;
    }
    
    if font_color[0].abs() 
        + (font_color[1] - 255).abs()
        + font_color[2].abs()
        <= 35 
    {
        count += 1;
    }
    
    if is_monospace == true {
        count += 1;
    }
    
    if count >= 2 {
        println!("L33T H4X0R")
    } else {
        println!("n00b")
    }
}
