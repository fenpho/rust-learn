// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }

use uuid::Uuid;

fn main() {
    // 生成一个新的 UUID
    let id = Uuid::new_v4();

    // 将 UUID 打印出来
    println!("Generated UUID: {}", id);
}
