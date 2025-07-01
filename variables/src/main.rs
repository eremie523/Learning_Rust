fn main() {
    // Variables Immutable
    let x = 12;
    println!("{x}");

    // x = 13; // Unable to change value or assign another value to this variable
    // println!("{x}");

    // Variables mutable
    let mut y = 12;
    println!("{y}");

    y = 13; // Why is a mutable variable
    println!("{y}");

    // Constants
    const NO_OF_DAYS_IN_A_YEAR: u32 = 7 * 4 * 12;
    println!("{}", NO_OF_DAYS_IN_A_YEAR);

    // NO_OF_DAYS_IN_A_YEAR = 12; // Cannot be reassigned a value
    // println!("{}", NO_OF_DAYS_IN_A_YEAR);

    // Shadowing
    let x = 24; // The immutable variable x can be shadowed to another value while maintaining its mmutability, It's basically creating another variable but naming it with the name of an existing variable to overshadow it
    println!("{}", x + 2); // Expressions can be passed in this fmt

    let y: char = 'j'; // Able to change the type of a variable whether mutable or immutable
    println!("{}", y);

    // const NO_OF_DAYS_IN_A_YEAR: i32 = -12; // Cannot shadow a constant, that's why it's a constant u get?? 😋
    // println!("{}", NO_OF_DAYS_IN_A_YEAR);
}
