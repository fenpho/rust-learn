fn main() {
    enum Coin {
        Penny(i32),
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> i32 {
        match coin {
            Coin::Penny(i) => {
                println!("Lucky penny!");
                i + 1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let penny = Coin::Penny(2021);
    let value = value_in_cents(penny);
    println!("value of coin is: {}", value);

    // #[derive(Debug)] // 使我们能够打印并观察各州的设计

    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     // --略 --
    // }
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }

    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         }
    //     }
    // }

    // value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
