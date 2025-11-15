// Vectors - Resizable arrays

use std::mem::size_of_val;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off last value
    numbers.pop();

    // Error - expected an array with a size of 5, found one with a size of 4
    // let numbers2: [i32; 5] = [1, 2, 3, 4];

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // -> Slice: [1, 2]

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
