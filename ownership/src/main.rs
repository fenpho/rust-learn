// fn main() { // 由于变量s还未被声明，所以它在这里是不可用的
//     let s = "hello"; // 从这里开始变量s变得可用
//     // 执行与s相关的操作
// } // 作用域到这里结束，变量s再次不可用

// fn main() { // 由于变量s还未被声明，所以它在这里是不可用的
//     let s = String::from("hello"); // 从这里开始变量s变得可用
//     // 执行与s相关的操作
// } // 作用域到这里结束，变量s失效

// fn main() {
//     let s = String::from("hello"); // 变量s进入作用域
//     takes_ownership(s); // s的值被移动进了函数
//                         // 所以它从这里开始不在有效
//     let x = 5; // 变量x进入作用域
//     makes_copy(x); // 变量x同样被传递进了函数
//                    // 但由于i32是Copy的，所以我们依然可以在这之后使用x
// } // x首先离开作用域，随后是s
//   // 但由于s的值已经发生了移动，所以没有什么特别的事情会发生

// fn takes_ownership(some_string: String) {
//     // some_string进入作用域
//     println!("{}", some_string);
// } // some_string在这里离开作用域，drop函数被自动调用
// // some_string所占用的内存也就随之被释放了

// fn makes_copy(some_integer: u32) { // some_integer进入作用域
//     println!("{}", some_integer);
// } // some_integer在这里离开了作用域，没有什么特别的事情发生

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership将它的返回值移动至s1中

//     let s2 = String::from("hello"); // s2进入作用域

//     let s3 = takes_and_gives_back(s2); // s2被移动进函数takes_and_gives_back中，而这个函数的返回值又被移动到了变量s3上
// } // s3在这里离开作用域并被销毁。由于s2已经移动了，所以它不会在离开作用域时发生任何事情。s1最后离开作用域并被销毁。

// fn gives_ownership() -> String { // gives_ownership会将它的返回值移动到调用它的函数内
//     let some_string = String::from("hello"); // some_string进入作用域
//     some_string // some_string作为返回值移动至调用函数
// }

// // takes_and_gives_back将取得一个String的所有权并将它作为结果返回
// fn takes_and_gives_back(a_string: String) -> String { // a_string进入作用域
//     a_string // a_string作为返回值移动至调用函数
// }

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of '{}' is '{}'", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len()会返回当前字符串的长度

//     (s, length) 
// }

fn main() {
    let s1 = String::from("hello");
    let len = calculute_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculute_length(s: &String) -> usize {
    s.len()
}