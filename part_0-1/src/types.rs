/*
 *  Primitive:
 *      Integers:
 *          u8, u16, u32, u64, u128
 *          i8, i16, i32, i64, i128
 *
 *      Floats:
 *          f32, f64
 *
 *      Boolean:
 *          bool
 *
 *      Character:
 *          char
 *
 *      Tuples
 *
 *      Arrays
 *
 */

pub fn run() {
    println!("\n====== TYPES =====\n");

    define();
}

fn define() {
    let x = 1; // default i32
    let y = 2.5; // default f64

    let z: i64 = 1212121212112;

    // Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max u128: {}", std::u128::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 > 1;

    let a1 = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1));
    println!("Is Greater {}", is_greater);
}
