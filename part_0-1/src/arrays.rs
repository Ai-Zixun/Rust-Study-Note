// Arrays - Fixed list where elements are the smae data type

use std::mem; 


pub fn run() {
    println!("\n====== ARRAYS =====\n");

    let mut numbers: [u32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("{:?}", numbers[0]);

    numbers[2] = 10;

    println!("{:?}", numbers);

    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated 

    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[u32] = &numbers[0..2]; 
    
    println!("{:?}", slice);

    
}
