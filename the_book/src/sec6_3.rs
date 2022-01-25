pub fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three")
    }

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count + 1,
    // }
    if let Coin::Quater(state) = coin {
        println!("State quater from {:?}", state)
    } else {
        count += 1;
    }
}
