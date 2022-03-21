pub fn run() {
    println!("Hello from the print.rs file");
    // Basic formatting
    println!("{} is {}", 1, "One");
    // Positional arguments
    println!("arg1{0}, arg2{1}, arg3{2} arg4{0}", "Zero", "One", "Two");

    // Named arguments
    println!(
        "🐈 {catName} is {yearsOld} years old"
        , catName = "Grizz"
        , yearsOld = 4
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic maths
    println!("{} + {} = {}", 10, 20, 10 + 20);
    println!("{} * {} = {}", 10, 20, 10 * 20);
    println!("{} - {} = {}", 10, 20, 10 - 20);
    println!("{} / {} = {}", 20, 10, 20 / 10);

}
