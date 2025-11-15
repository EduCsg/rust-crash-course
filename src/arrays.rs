// Fixed list where elements are the same data type

use std::mem::size_of_val;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Error - expected an array with a size of 5, found one with a size of 4
    // let numbers2: [i32; 5] = [1, 2, 3, 4];

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // -> Slice: [1, 2]
}
