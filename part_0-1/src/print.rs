pub fn run() {
    // Print to console
    println!("Hello from the print.rs");

    // Basic Formatting
    println!("Number: {}", 8u32);
    println!("{} + {} = {}", 1, 1, 1 + 1);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "football"
    );

    // Placeholder traits
    println!("Binary: {:b} \nHex {:x}\nOctal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, 10u32, "Hello"));
}
