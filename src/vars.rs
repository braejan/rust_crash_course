// Variables hold primitive data or references to data
// Variables ar immutable by default
// Rust is a block-scoped language.
pub fn run() {
    let name = "Braejan";
    let mut age = 31;
    println!("My name is {} and I was {} years old last month", name, age);
    age = 32;
    println!("My name is {} and I'm {} years old.", name, age);

    // Define constant (uppercase)
    const EULER: f64 = 0.5772156649;
    println!("euler constant is {}", EULER);

    // Assign multiples vars

    let(cat_name, cat_color) = ("Grizz", 0x964B00);
    println!("Cat name: {} \nHex color: {}", cat_name, cat_color);
}
