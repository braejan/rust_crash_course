/*
 * Primitive types:
 *  Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128
 *  Floats: f32, f64
 *  Boolean: bool
 *  Characters: char
 *  Tuples
 *  Arrays
 */

/* Rust is a statically typed language, which means that it must know the 
 * types of all variables at compile time, however, the compiler ca usually
 * infer what type we want to use based on the value and how we use it.
 */
pub fn run() {
    // Default is "i32"
    let age = 32;
    println!("Age: {age}", age = age);
    // Default is "f64"
    let euler = 0.5772156649;
    println!("Euler: {}", euler);
    // Add explicit type
    let x: u8 = 0;
    println!("x={}", x);
    // Find max size
    println!("Max i32:{}", std::i32::MAX);
    println!("Max i32:{}", std::i64::MAX);
    // Boolean
    let mut is_active = true;
    println!(
        "Initial status: {status}",
        status = is_active
    );
    is_active = false;
    println!(
        "Final status: {status}",
        status = is_active
    );

    // Get boolean from expression
    let is_grater = 15 > 30;
    println!("Debugging 1: {:?}", is_grater);

    let icon = '\u{1f92f}';
    println!("{} -> I'm learning rust.", icon);
}
