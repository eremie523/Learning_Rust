fn main() {
    let x: u32 = 21;
    let y: u32 = x;

    // In the instance above we assume that the value of x is copied into x, which is totally correct.
    // In rust we utilize two types of memory; the stack and the heap, the stack holds all the necessary data while the heap holds referenced data
    // All data on the stack must have a known size at compile time, and if that's the case, how do we make stuffs like dynamic array or stuffs like accepting user inputs that the sizes may vary??
    // To do that rust utilizes the heap, so when a dynamic sized variable is made, the memory allocator goes to the heap and looks for a location that is big enough to contained the data that it's initialized with eg String::from("Hello"), looks for a memory location big enough to contain Hello
    // and then a the value or data is fit in there and a pointer to that data is layed up in the stack.
    // Now the stacks data have to be of known length because of the system at which it works, it uses a push ad pop method, so maybe each word length is 32bits and then to access the value at the top of the stack it just pops out the first 32 bits because of the known size, it doesn't have to think if it fetched the proper content or if the offsetting due to the previous pop actions have taken out some chunks of the variable or data trying to be accessed atm making it extremely fast in comparison to heap that has to do an allocation and bookmarking of memory locations.
    // Note: It's called a stack cuz it follows the stack pattern eg like a pile of plate, you add new contents to the top and take out contents from the top ie, first in last out or last in first out
    // These approach helps for things like functions where all the parameters and variables within a function scope are pushed into the stack and are being used and when they're done are popped out one after the other out of the stack helping for code trace.


    // An example of a complex data type as such that would be the String type 
    // Note its not the same as the string literal type oh, that is &str

    let mut x: String = String::from("Hello");
    x.push_str(", world"); // Here we can see that the size of x increase from 5 bytes to 12 bytes

    println!("{:?}", x);

    // For operations like these that utilizes the stack, if we do this
    let y = x; // We might expect the same thing that happened for stack values to happe here but a totally different things, yes a copy is done but it's more of a shallow copy that is the actual data on the heap isn't copied, just the meta object containing the pointer leaving on the stack that exist
    // In this, it makes two pointers to the same memory location, now it comes to rusts pattern of memory management by clearing out unexisting data, in some languages, there's something called a garbage collector that frequently runs to clear out unused memory, and some other languages you have to explicitly drop the memory location which can lead to errors.
    // In rust, when a value (variable) goes out of scope it's automatically dropped eg
    {
        // s doesn't exist as it has not been initialized
        let s = 32; // s is now valid

        // s can be used all within here
    } // s goes out of scope and that value (VARIABLE) is dropped, that is. s no longer exists.
    // Now if we then have a double pointer to the same memory on the heap, if both of the variables go out of scope, they'll both try to drop the same data on the heap, and that's not possible the data after being dropped by one of the variables wouldn't exist for the other variable to drop and when the other variable attemps, it might delete out a wrong information or break the memory or introduce a memory bug, this action is called a double free error.
    // To fixx this, after a complex variable as such is copied, when the new pointer is made to that heap location, the old pointer is deleted ( considered ) invalid and can't be used anymore, that way we always have only one stack pointer (owner) to that data
    // Therefore here, x is invalid. this is called a move instead of a copy action
    // println!("{:?}", x); // This will produce a compilation error;
    
    // To create a copy of both the stack pointer and the heap value, you have to explicity call the clone method on the variable eg
    let x = y.clone(); // This isn't always good because it's memory consuming for large data chunks and will cost alot of processing power for some operations like if ( x == y ) since the entire bulky heap content has to be mapped to the other to compare

    // Ownership in functions
    // Same thing happens when assigning a value as a parameter to a function
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    // And same thing for returns
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1); // An extra shadowing expression and a bigger return value, this introduces the use of references

    println!("The length of '{s1}' is {len}."); 

}// Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function                                      
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}


// A limitation, seeing that whatever value we pass into a function, we have to pass it out causing an extra overhead and a bigger return structure
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}