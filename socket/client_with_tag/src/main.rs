// 客户端

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, from_str};

const DATA_BEGIN: &str = "feng-data-begin";
const DATA_END: &str = "feng-data-end";

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.expect("Failed to connect to server");
    println!("Connected to server: 127.0.0.1:8080");

    // 构造数据
    let json_data = json!({"key": "value", "name": "小明"});
    let data = format!("{}{}{}", DATA_BEGIN, json_data.to_string(), DATA_END);

    // 发送数据
    if let Err(e) = stream.write_all(data.as_bytes()).await {
        eprintln!("Failed to send data: {}", e);
        return;
    }

    // 接收回应
    let mut buffer = Vec::new();
    loop {
        let mut chunk = vec![0; 1024];
        let size = stream.read(&mut chunk).await.unwrap();
        buffer.extend_from_slice(&chunk[..size]);

        // 检查是否包含结束标志
        if buffer.ends_with(DATA_END.as_bytes()) {
            break;
        }
    }

    // 转换为字符串
    let response = String::from_utf8_lossy(&buffer);

    // 检查开始和结束标志
    if response.starts_with(DATA_BEGIN) && response.ends_with(DATA_END) {
        // 提取 JSON 数据部分
        let json_data = &response[DATA_BEGIN.len()..(buffer.len() - DATA_END.len())];
        let json_value: Result<serde_json::Value, _> = from_str(json_data);

        match json_value {
            Ok(parsed_json) => {
                println!("Server response JSON: {:?}", parsed_json);
            }
            Err(e) => {
                eprintln!("Error parsing JSON in server response: {}", e);
            }
        }
    } else {
        eprintln!("Invalid data format received in server response");
    }
}
