use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // 连接到服务端
    let addr: std::net::SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let mut stream = TcpStream::connect(&addr).await.unwrap();
    println!("Connected to server: {}", addr);

    let data_to_send = json!({"name": "John", "age": 30});
    let data_to_send_json = serde_json::to_vec(&data_to_send).unwrap();
    stream.write_all(&data_to_send_json).await.unwrap();

    // 接收服务端的JSON响应
    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&mut buffer).await.unwrap();
    let received_data: Value = serde_json::from_slice(&buffer[..n]).unwrap();
    println!("Received data from server: {:?}", received_data);
}
