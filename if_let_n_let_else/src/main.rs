#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska
}

impl UsStates {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsStates::Alabama => year >= 1819,
            UsStates::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates)
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("{max}"),
        _ => println!("Something's off")
    };

    // A replacement fpr this
    if let Some(max) = config_max {
        println!("{max}"); // Exists within the if le block
    } else { // Has an optional else block
        println!("Something's off");
    };

    // println!("{max}"); // Max doesn't exist outside the if let block;

    fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
}