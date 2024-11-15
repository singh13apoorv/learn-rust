fn main() {
    // Two most used string types in Rust are String and &str.

    // A String is stored as a vector of bytes(Vec<u8>), but guaranteed to always be a valid
    // UTF-8 sequence. String is heap allocated, growable and not null terminated. (I know all
    // these can sound intimediating but we will learn about everything as we go.)

    //&str is a slice(&[u8]) that always points to a valid UTF-8 sequence, and can be used to view
    //into a String, just like &[<T>] is a view in Vec<T>.(Don't worry for now, we will learn about
    //vectors and all.)

    let greeting: String = String::from("Hello world!");
    println!("{}", greeting);

    let char1: Option<char> = greeting.chars().nth(2);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("You are trying to access unassigned memory."),
    }

    // String in Rust is very complex and we would add more code and concepts here its not over
    // yet.
}
