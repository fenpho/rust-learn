use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, from_str};

const DATA_BEGIN: &str = "feng-data-begin";
const DATA_END: &str = "feng-data-end";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind address");
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_client(&mut socket).await;
        });
    }
}

async fn handle_client(stream: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size > 0 => {
                let data = String::from_utf8_lossy(&buffer[0..size]);
                println!("Received Raw Data: {:?}", data);

                // 检查开始和结束标志
                if data.starts_with(DATA_BEGIN) && data.ends_with(DATA_END) {
                    // 提取 JSON 数据部分
                    let json_data = &data[DATA_BEGIN.len()..(size - DATA_END.len())];
                    match from_str::<serde_json::Value>(json_data) {
                        Ok(parsed_json) => {
                            println!("Received JSON: {:?}", parsed_json);
                        }
                        Err(e) => {
                            eprintln!("Error parsing JSON: {}", e);
                        }
                    }

                    // 处理数据，这里简单地回显
                    let response = json!({"key": "value"});
                    let data = format!("{}{}{}", DATA_BEGIN, response.to_string(), DATA_END);

                    // 发送回应
                    if let Err(e) = stream.write_all(data.as_bytes()).await {
                        eprintln!("Error sending response: {}", e);
                    }
                } else {
                    eprintln!("Invalid data format received");
                }
            }
            Ok(_) | Err(_) => break,
        }
    }
}
