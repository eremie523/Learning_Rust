#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width = 30;
    let height = 50;

    let rect: Rectangle = Rectangle {
        width,
        height
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!("rect is {rect:#?}");
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}