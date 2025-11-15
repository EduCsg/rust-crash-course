pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Only can print directly String values, to print another value, use placeholders
    println!("{}", 1);

    // Multiple placeholders - fill by index
    println!("{} is from {}", "Brad", "Mass"); // -> Brad is from Mass

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    ); // -> Brad is from Mass and Brad likes to code

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    ); // -> John likes to play Baseball

    // Placeholder traits
    println!("Binary {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10); // -> Binary 1010, Hex: a, Octal: 12

    // Placeholder for debug trait
    println!("{:?}", (12, true, "test")); // -> (12, true, "test")

    // Basic math
    println!("10 + 10 = {}", 10 + 10); // -> 10 + 10 = 20
}
