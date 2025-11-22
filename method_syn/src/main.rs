#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // Methods. N/B: the &self here is self: &self eg rectangle: &Rectangle, we can also have &mut self or self alone
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods with more parameter
    fn scale(&mut self, factor: u32) {
        (*self).width = self.width * factor;
        (*self).height = self.height * factor;
    }

    // Associative functions or static methods
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }
}

// Multiple Impl block
impl Rectangle {
    fn can_hold(&self, anotherRect: &Self) -> bool {
        (self.width > anotherRect.width) && (self.height > anotherRect.height)
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40
    };

    let rect2 = rect.clone();

    println!("The area of rectangle is {}", rect.area());

    let mut rect = Rectangle::new(40, 50);

    println!("The area of rectangle is {}", rect.area());

    rect.scale(3);

    println!("The area of rectangle is {}", rect.area());

    println!("can {rect:#?} \nhold {rect2:#?}: \n{}", rect.can_hold(&rect2));
}

