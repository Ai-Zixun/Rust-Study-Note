// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let name = "Crusoe";
    let mut age = 10;

    println!("The name is {name}", name = name);
    println!("The age is {age}", age = age);

    age += 1;

    println!("The age is {age}", age = age);

    // Define constant
    const ID: i32 = 0xFF;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Zach", 1);
    println!("{} is {}", my_name, my_age);
}
