/*
    Primitive Types
    Integers    - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory)
    Floats      - f32, f64
    Boolean     - (bool)
    Characters  - (char)
    Tuples
    Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all
// variables at compile time, however, the compiler can usually infer what type we want
// to use based on the value and how we use it.

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 123123123123;

    // Find max size
    println!("Max i32 {}", std::i32::MAX); // -> 2147483647
    println!("Max i32 {}", std::i64::MAX); // -> 9223372036854775807

    // Boolean
    let is_active: bool = true;

    // Get a boolean from a expression
    let is_greater = 10 < 5;

    println!("{:?}", (x, y, z, is_active, is_greater));

    // Char -> needs to be only 1 character with single quotes
    let a1 = 'a';
    let face = '\u{1F600}'; // -> 😀

    println!("{:?}", (a1, face));
}
