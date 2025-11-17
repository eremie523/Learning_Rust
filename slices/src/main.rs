fn main() {
    println!("Hello, world!");

    let s: String = String::from("Hello world!");
    println!("{:?}", first_word(&s));
}

// Test problem: A function that returns the first word of a string
fn first_word<'a>(s: &'a String) -> &'a str { // Use a reference so we don't take ownership of the word
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}