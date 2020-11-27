fn main() {
    hello(); 
    println!("Hello, world!");
}


fn hello() {
    let num: u32 = 123; 
    let mut change: i32 = -123; 
    change = change - 100; 

    println!("New var {}", num);
    println!("New var {}", change);
}