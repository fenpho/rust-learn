// fn main() {
//     // 定义
//     let num: u32 = 20;
//     let num1 = 98_100;
//     let num2 = 0xff;
//     let num3 = 0o77;
//     let num4 = 0b1111_0000;
//     let singlechar = b'A';

//     let num5 = 0.5;
//     let num6: f32 = 0.5;

//     let t = true;
//     let f: bool = false; // 附带了显式类型标注的语句

//     let c = 'z';

//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {}", y);

//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;

//     let a = [1, 2, 3, 4, 5];

//     let a = [1, 2, 3, 4, 5];
//     let first = a[0];
//     let second = a[1];

//     println!("Hello, world!, {},\n {}", num1, singlechar);

//     // 运算
//     // 加法
//     let sum = 5 + 10;
//     // 减法
//     let difference = 95.5 - 4.3;
//     // 乘法
//     let product = 4 * 30;
//     // 除法
//     let quotient = 56.7 / 32.2;
//     // 取余
//     let remainder = 43 % 5;
// }

// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     println!("Hello, world!");
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x)
// }

// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn five() -> i32 {
//     5
// }
// fn main() {
//     let x = five();
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {}", x);
// }
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {}", x);
// }
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }

// fn main() {
//     another_function(5, "Hi");
// }

// fn another_function(x: i32, y: &str) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}

// So we’re doing something complicated here, long enough that we need 
// multiple lines of comments to do it! Whew! Hopefully, this comment will 
// explain what’s going on.

// I’m feeling lucky today. 
let lucky_number = 7;