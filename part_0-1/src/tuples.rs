// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    println!("\n====== TUPLES =====\n");

    let person: (&str, &str, u8) = ("Crusoe", "Haruto", 10);

    println!("{} {} {}", person.0, person.1, person.2);
}
