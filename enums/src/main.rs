// #[derive(Debug)]
// enum UsState {
//     Albama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         },
//     }
// }


// fn main() {
//     let coin = Coin::Penny;
//     let value = value_in_cents(coin);
//     println!("The value of the coin is: {} cents", value);
//     let second_value = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("The value of the coin is: {} cents", second_value);
// }


use std::fs::File;

fn main() {
    let res = File::open("foo");
    match res {
        Ok(f) => { /* do stuff */ },
        Err(e) => { /* do stuff */ },
    }
}

