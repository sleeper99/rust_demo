// Variables hold primitive data or references to data\
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Anna";
    // Using `mut` to make variables mutable
    let mut age = 18;
    age = 19;
    println!("Name is {} and I am {}", name, age);

    // Define constant (must defind the variable type)
    const ID: u32 = 001;
    println!("ID: {}", ID);

    // Assign mutiple vars
    let (my_name, my_age) = ("Lee", 23);
    println!("{} is {}", my_name, my_age);
}
