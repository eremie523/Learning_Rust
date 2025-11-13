fn main() {
    // Default - Sequencial Structure

    // Selection structure
    using_if_statements(33);
    using_extended_ifs(3);
    using_match_statements("JS".to_string());

    // if as a an expression wrapper
    let y = 30;
    let x = if y == 30 { y/3 } else { y }; // The returns must be of the same type so the compiler can know the type of x at compile time

    // Iterative Structure
    // using_a_loop();
    using_a_controlled_loop_with_break_and_continue_keywords();
    using_a_while_loop(&mut 0);
    looping_through_a_collection([1, 2, 3, 4, 7, 6, 10, 12, 3, 19]);
}

fn using_if_statements(x: u32) {
    if x > 32 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}

fn using_extended_ifs(x: u8) {
    if x < 2 {
        println!("Too small")
    } else if x == 2 {
        println!("Just perfect");
    } else {
        println!("Too big");
    }
}

fn using_match_statements(x: String) {
    match x {
        val if val == "JS".to_string() => println!("Very short abbreviations of JohnnyScripts ie JS"),
        val if val == "JScripts".to_string() => println!("Abbreviation of alias JohnnyScripts"),
        val if val == "JohnnyScripts".to_string() => println!("Actual alias"),
        _ => println!("Not any of my alias")
    }
}

fn using_a_loop() {
    loop {
        println!("Infinite Looping");
    }
}

fn using_a_controlled_loop_with_break_and_continue_keywords() {
    let mut count: u32 = 0;
    'controlled_loop: loop {
        if count == 10 {
            // Breaks can also be used to return a value
            break 'controlled_loop;
        }

        if count == 15 {
            continue 'controlled_loop;
        }

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");
        
        let input = input
            .trim();

        if input != "" { 
            count = input
                .parse::<u32>()
                .expect("Unable to parse input");
        } else {
            println!("Empty input");
        }
    }
}

fn using_a_while_loop(x: &mut u32) {
    while *x <= 10 {
        println!("Looping..., x is {}", x);
        *x+=1;
    }
}

fn looping_through_a_collection(x: [u32; 10]) {
    for item in x {
        println!("Item in x is {}", item)
    }
}