// 注意依赖中要申明v4，uuid = { version = "0.8.1", features = ["v4"] }
use uuid::Uuid;

fn main() {
    // 生成一个新的 UUID
    let id = Uuid::new_v4();
    let bytes = id.as_bytes();

    // 将 UUID 打印出来
    println!("Generated UUID: {:?}", bytes);
    let hex_string = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("16 Byte Hex String: {}", hex_string);
}
