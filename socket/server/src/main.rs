use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // 启动服务端
    let addr: std::net::SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on: {}", addr);

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_client(stream));
        }
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0u8; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => break, // 连接关闭
            Ok(n) => {
                // 从客户端接收JSON数据
                let received_data: Value = serde_json::from_slice(&buffer[..n]).unwrap();
                println!("Received data: {:?}", received_data);

                // 从服务端发送JSON数据
                let response_data = json!({"message": "Hello from server!"});
                let response_json = serde_json::to_vec(&response_data).unwrap();
                stream.write_all(&response_json).await.unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }
}
