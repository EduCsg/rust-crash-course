// Variables hold primitive data or references to the data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Eduardo";
    let mut age = 21;
    println!("My name is {} and I'm {}", name, age);
    age = 22;
    println!("My name is {} and I'm {}", name, age);

    // Define Constant
    // When you have a const
    // -> variable name needs to be UPPER_SNAKE_CASE
    // -> you always need to define the variable type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Eduardo", 21);
    println!("{} is {}", my_name, my_age)
}
