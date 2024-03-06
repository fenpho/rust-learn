// fn main() {
//     // 变量与可变变量
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);

//     // 隐藏
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);

//     // 通过隐藏改变变量类型
//     let spaces = "     ";
//     let spaces = spaces.len();
//     println!("The value of x is: {}", spaces);
// }
// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
// }
// fn main() {
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
//     let t = true;
//     let f:bool = false; // 附带了显式类型标注的语句
// }

// fn main() {
//     let c = 'z';
//     let z = '♬';
//     let heart_eyed_cat = '😼';
// }

// fn main() {
//     let tup: (i32, f64, i32) = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("the value of y is: {}", y);
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;
//     println!("the value of five_hundred is: {}", five_hundred);
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//     // let a: [i32; 5] = [1, 2, 3, 4, 5];
//     // let a = [3; 5];

//     let first = a[0];
//     let second = a[1];
//     println!("the value of five_hundred is: {}, {}", first, second);
// }
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("the value of element is: {}", element);
}
