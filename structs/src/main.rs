// Create a struct
#[derive(Debug)] // This trait implementation is optional so that the struct can be formatted and printed out with a println functions
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // Instantiate a struct
    let user: User = User {
        username: "Jscripts".to_string(),
        email: "reremie523@gmail.com".to_string(),
        active: true,
        sign_in_count: 0
    };

    // Access values // The user struct is immutable and cannot be modified
    println!("{:?}", user.email);

    let mut user2: User = User {
        username: String::from("JS"),
        email: String::from("johnsoneremie@gmail.com"),
        active: false,
        sign_in_count: 0
    };

    // Modified a property of the user2 object which is an instance of the User struct
    user2.active = true;

    println!("{}", user2.active);

    println!("{:#?}", build_user(String::from("jeffjossy16@gmail.com"), String::from("JJ")));

    let user3: User = User {
        email: "bot@gmail.com".to_string(),
        ..build_user(String::from("jeffjossy16@gmail.com"), String::from("JJ"))
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}