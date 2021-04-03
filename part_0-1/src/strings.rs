pub fn run() {
    println!("\n====== STRINGS =====\n");

    // Primitive str = immutable fixed-length string somewhere in memory
    // String = Growable, Heap-allocated data structure

    let foo: &str = "Hello";
    let mut bar: String = String::from("World");

    bar.push('!');
    bar.push_str(" LOL 你好");

    println!("{} {}", foo, bar);

    println!("Capacity: {}", bar.capacity());
    println!("Length: {}", bar.len());

    let a: char = '是';
    println!("{}", a);

    println!("Length: {}", a.len_utf8());

    for word in "a b c d e".split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Asertion
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
