fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    println!("Hello, world!");

    // 值无效或缺失
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // mod front_of_house {
    //     pub mod hosting {
    //         pub fn add_to_waitlist() {}
    //     }
    // }
    // use crate::front_of_house::hosting;
    // pub fn eat_at_restaurant() {
    //     hosting::add_to_waitlist();
    //     hosting::add_to_waitlist();
    //     hosting::add_to_waitlist();
    // }
    // // # fn main() {}
    // mod front_of_house {
    //     pub mod hosting {
    //         pub fn add_to_waitlist() {}
    //     }
    // }
    // use self::front_of_house::hosting;
    // pub fn eat_at_restaurant() {
    //     hosting::add_to_waitlist();
    //     hosting::add_to_waitlist();
    //     hosting::add_to_waitlist();
    // }
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
}
