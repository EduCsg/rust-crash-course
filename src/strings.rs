// String Types
// str      -> Immutable, fixed-length, somewhere on memory'
// String   -> Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // let hello_str = "Hello Str";
    let mut hello_string = String::from("Hello, ");

    println!("Length -> {}", hello_string.len());

    // push a char
    hello_string.push('W');

    //push a string
    hello_string.push_str("orld");

    println!("{}", hello_string);

    // Capacity in bytes
    println!("Capacity: {}", hello_string.capacity());

    // Check if empty
    println!("Is Empty: {}", hello_string.is_empty());

    // Contains
    println!("Contains 'World': {}", hello_string.contains("World"));

    // Replace
    println!("Replace: {}", hello_string.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
