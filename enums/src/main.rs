enum IPAddrKind {
    v4,
    v6
}

enum IP {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16)
}

// Another enum
enum Command {
    Stop, // Has no associated data
    Start, // Has no associated data
    Move { x: u32, y: u32 }, // Is associated with a named fields like a struct
    ChangeWeaponColorRGB (u32, u32, u32), // Associated with unnamed fields like a turple
    Speak (String) // Associated with a string
}

// If we're to define with structs we'll have something like this
struct Stop;
struct Start;
struct Move {
    x: u32,
    y: u32
}
struct ChangeWeaponColorRGB (u32, u32, u32);
struct Speak (String);

// We're seeing the redundancy loss by using an enum bcuz we don't have to repeat struct everytime and also there's a form of grouping in the enum in comparison to that of the struct which are totally independent identities

// Another similarity between enums and structs is we can define methods and assiciative methods for both
impl Command {
    fn call(&self, cmd: &Command) {
        // Carry out the function implementation here, also we can accept any of the enums variant with one type cover which is something we can't do with independent structs
    }
}

// The built in option enum - This enum helps us have a variant that acts as a null type 
// An example of how the standard library implementation looks like is
// enum Option<T> {
//     None,
//     Some(T)
// }

fn main() {
    let ipAddr1 = IP::V4(126, 128, 5, 10);
    let ipAddr2 = IP::V6(255, 255, 10, 90, 123, 188);

    let cmd = Command::Start;
    let cmd2 = Command::Speak(String::from("Testing an action"));

    cmd.call(&cmd2);

    // let s = Option::Some("Hello world");
    // let no_s: Option<u32>  = Option::None;

    // Now using the option enum from the standard library which is used even in built in method impl
    let x = Some(32); // Type annotation not necessary, compiler will do that automatically
    let y: Option<char> = Some('c'); // Type annotation not necessary, compiler will do that automatically
    let z: Option<bool> = None; // Necessary to explicity state the Option<bool> type so the generic type variable T will know what type it represents, in this case a boolean

    // Also notice that we don't need to explicitly include the Options enum from it's parent module and same with it's variants?? That's because it's included in the standard prelude that is automaticall included in all rust programs, this shows how important the option enum is

    // N/B why this form (option) is so good is becuz we can't accidentally use this form or object as if it had a definite value so we can't make a mistake of assuming that a value that returns either null or smth will not be null and use it in an operation which can lead to bugs
    // let x = 32 + Some(32); // Even if we have a value of i32 here adding with an i32, rust has it in mind that Some is a variant of Option so as Option<u32> maybe coming as a parameter won't annotate that there's a value abd break the program
    // To carry out this we have to first extract the value after being certain that it's a Some ie has a value
    let Some(f) = Some(32) else { panic!("Null value provided") };
    let x = 32 + f;
}
