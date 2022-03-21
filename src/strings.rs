/*
 * Primitive srt = Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure - Use when you
 * need to modify or own String data.
 *
 */
pub fn run(){
    let mut greetting = String::from("Hello fellow kids! ");
    greetting.push('\u{1F485}');
    println!("Gretting: {}", greetting);
    let mut text = String::from(format!("\n{} -> this was added later.", '\u{1F60F}'));
    greetting.push_str(&mut text);
    println!("Second greetting: {}", greetting);
    // Capacity in bytes 
    println!("Capacity: {}", greetting.capacity());
    // Get length
    println!("Lenght: {}", greetting.len());
    // Is empty
    println!("Is empty: {}", greetting.is_empty());
}
