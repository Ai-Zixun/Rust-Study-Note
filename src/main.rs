use std::mem; 

const MEANING_OF_LIFE:u8 = 11; 
static mut GLOBAL:u16 = 15; 

fn main() {
    hello(); 
    operators(); 
    scope(); 
    println!("Hello, world!");

    println!("{}", MEANING_OF_LIFE);
    unsafe {
        println!("{}", GLOBAL);
    }

}

fn hello() {
    let num: usize = 123; 
    let mut change: i16 = -123; 
    let large: u64 = 100000000; 
    change = change - 100; 

    println!("var {}, size of {}", num, mem::size_of_val(&num));
    println!("var {}, size of {}", change, mem::size_of_val(&change));
    println!("var {}, size of {}", large, mem::size_of_val(&large));

    let d: char = 'x';
    println!("var {}, size of {}", d, mem::size_of_val(&d));

    let f: f32 = 3455.12;
    println!("var {}, size of {}", f, mem::size_of_val(&f));

    let b: bool = false;
    println!("var {}, size of {}", b, mem::size_of_val(&b));


}

fn operators() {
    let mut a = 2 + 4 * 4; 
    a += 1;
    println!("{}", a);
    println!("{}", 54 / 2);
    println!("{}", 45 % 2);
    println!("{}", 54 + 2);

    println!("{}", i64::pow(10, 4));
    println!("{}", f64::powi(1.12, 4));
    println!("{}", f64::powf(10.0, std::f64::consts::PI));
}

fn scope() {
    let mut i:u64 = 0; 
    
    {
        let b = 444;
        i += b; 
    }

    println!("Scope Result: {}", i);
}