use std::collections::{HashMap};
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9493").await?;

    let mut clients = HashMap::<SocketAddr, TcpStream>::new();

    println!("Server listening on port 9493");

    loop {
        let (mut socket, _) = listener.accept().await?;
        clients.insert(socket.peer_addr()?, TcpStream::from(&socket));

        println!("peer_addr{:?}", socket.peer_addr());

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // 读取客户端发送的数据
            let num_bytes = socket.read(&mut buffer).await.unwrap();
            let request = String::from_utf8(buffer[..num_bytes].to_vec()).unwrap();
            println!("Received request: {}", request);

            // 处理请求
            let response = handle_request(request);

            // 发送响应给客户端
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to write response: {}", e);
            }
        });
    }
}

fn handle_request(request: String) -> String {
    // 处理请求，返回响应
    format!("Hello, {}!", request)
}