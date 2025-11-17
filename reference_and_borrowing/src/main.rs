fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    change(&mut s1);
}

// Now let's consider our calculate length function from the ownership sect, for referencing we simply have to do this
fn calculate_length(s: &String) -> usize {
    s.len()
}
// Here we didn't need to provide a tuple structure, code becomes easier, lighter and faster
// A reference is like a pointer, it's basically a memory address that can be followed to a particular value and unlike a pointer, a reference must always have a value

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Note no two mutable reference can exist at the same time in a scope ie one has to be invalidated immediately the second one is made and if you try to call the first after the second has been made, it shows the compiler that it's stil in use and then it brings about to mutable reference causeing a code crash
// the reason why multiple write references aren't allowed is to prevent concurrency issues ( ie one reference making a change to the same heap and possibly stack memory while the other too is doing same)
// we also cant have a read and write pointer value to the same variable and at the same time
fn dangle() -> &String {
    let s = String::from("hello");

    &s // yiu cant pass out a reference to a value that the owner has died
}