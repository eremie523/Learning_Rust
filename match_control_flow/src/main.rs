#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates)
}

fn main() {
    let coin: Coin = Coin::Quarter(UsStates::Alabama);

    println!("The value of coin is: {} cents", getCoinValueInCents(&coin));
}

fn getCoinValueInCents(coin: &Coin) -> u32 {
    match coin {
       Coin::Penny => 1,
       Coin::Nickel => 5,
       Coin::Dime => 10,
       Coin::Quarter(state) => {
        println!("This is the {state:?} quarter");
        100
       } 
    }
}

fn printAnOptionalValueIfExists<T: std::fmt::Debug>(result: &Option<T>) {
    match result {
        Some(val) => println!("{val:?}"),
        None => {},
        // _ => println!("Nothing happens - default case")
    };
}

// Nb matched are exhaustive so you have to match every possible outcome, to match other unknown conditions or excluded conditions, you can simply use the _; eg above or 
fn diceShii(roll: u8) {
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    };

    fn add_fancy_hat() {};
    fn remove_fancy_hat() {};
    fn move_player(num_spaces: u8) {};
}
