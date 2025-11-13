fn main() {
    println!("another_function says: {:?}", another_function());
    println!("add function output: {}", add(3, 4));

    // Statements vs expression; a statement has no return, while an expression has a return eg
    // let x = (let y = 2); // let is a statement and has no return to give to the next let statement
    let x = {
        println!("Hello world");
        3
    };
}

fn another_function() -> String {
    return "Another function".to_string();
}

fn add(x: u32, y: u32) -> u32 {
    x+y
}
