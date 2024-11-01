// 1. Try to compile this code;
// 2. Fix the errors
// 3. Create your own Option<T>

fn main() {
    // Using Option with i32
    let some_number = Some(42);
    let none_number = None;

    // Using Option with String
    let some_text = Some(String::from("Hello, Rust!"));
    let none_text = None;

    // Working with Option<i32>
    match some_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("We don't have a number"),
    }

    match none_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("We don't have a number"),
    }

    // Working with Option<String>
    match some_text {
        Some(ref value) => println!("We have text: {}", value),
        None => println!("We don't have text"),
    }

    match none_text {
        Some(ref value) => println!("We have text: {}", value),
        None => println!("We don't have text"),
    }
}
