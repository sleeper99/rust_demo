/*
Primitive Types
IntegersL: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits the take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays (fixed length)
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, howevedr, the compiler can usually infer what ype we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 1.6;

    // Add explicit type
    let z: i64 = 123456789;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater:bool = 10 == 5;

    let a1 = 'a';
    let face = '\u{1F609}';
    
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}