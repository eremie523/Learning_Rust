fn main() {
    // Let's note, Rust is a statically typed language, which means that all variables must have a known type at compile time.
    
    // We have two categories of datatypes - primitive(scalar) and compound

    // Scalar types
    // Numbers
    // Integers
    // Unsigned Integers
    let x: u8 = 32; // Max value - (2^8 - 1), Min value - 0
    let x: u16 = 32; // Max value - (2^16 - 1), Min value - 0
    let x: u32 = 32; // Max value - (2^32 - 1), Min value - 0
    let x: u64 = 32; // Max value - (2^64 - 1), Min value - 0
    let x: u128 = 32; // Max value - (2^128 - 1), Min value - 0
    let x: usize = 32; // Max value - (2^(word-length of the os eg 64 for x64 engines) - 1), Min value - 0

    // Signed Integers
    let x: i8 = 32; // Max value - ((2^8)/2 - 1), Min value - negative (2^8)/2
    let x: i16 = 32; // Max value - ((2^16)/2 - 1), Min value - negative (2^16)/2
    let x: i32 = 32; // Max value - ((2^32)/2 - 1), Min value - negative (2^32)/2
    let x: i64 = 32; // Max value - ((2^64)/2 - 1), Min value - negative (2^64)/2
    let x: i128 = 32; // Max value - ((2^128)/2 - 1), Min value - negative (2^128)/2
    let x: isize = 32; // Max value - ((2^(word-length of the os eg 64 for x64 engines))/2 - 1), Min value - negative (2^(word-length of the os eg 64 for x64 engines))/2

    // Floating points ie decimals
    let x: f32 = 12.04;
    let x: f64 = 125.64;

    // Note: Number literals can be written in various forms, an example of a number literal is 3543, the forms they can be written are
    // Decimals - Default type, human readable base 10
    let x: u32 = 43;
    // Hexadecimals - Computer readable for high level systems (prefixed with 0x) - Base 16 form
    let x: u32 = 0x9ff;
    // Octal - Base 8 form (Prefixed with 0o)
    let x: u32 = 0o734;
    // Decimals - Base 2 form (Prefixed with 0b)
    let x: u32 = 0b11010;
    // Bytes - for u8 datatypes only
    let x: u8 = b'A';


    // Boolean type - True false value;
    let x: bool = true;

    // Character type - A single unicode character, can also hold emojis, 4 bytes in size which is 32 bits;
    let x: char = 'A';

    


    // Compound Types
    // Lists
    let x: [char; 8] = ['J', 'S', 'c', 'r', 'i', 'p', 't', 's'];
    let x: [bool; 3] = [true, false, true];
    let x: [u32; 4] = [4; 4]; // [4, 4, 4, 4]

    // Accessing values by index;
    println!("{:?}", &x[3]);
    println!("{:?}", &x[0..3]);

    // Tuples
    let x: (u32, char, bool) = (283, 'J', true);

    // Accessing  by index;
    println!("{}", x.1);

    // By destructuring;
    let (a, b, c) = x;
    println!("{}", c);


    // Note: the concept for reinitializing a variable again to change it's datatype or it's value is called shadowing, it doesn't modify the original variable, it only makes a new one to cover up the old one;

    // Excercise 
    let array = [32, 75, 29, 13, 82, 14];

    println!("Pick an array index to print result");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to get input");

    let index = input
        .trim()
        .parse::<usize>()
        .expect("Unable to parse input");

    println!("{}", array[index]);
}
