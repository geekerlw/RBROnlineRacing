use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9493").await?;

    loop {
        let request = "World";

        // 发送请求给服务器
        if let Err(e) = stream.write_all(request.as_bytes()).await {
            eprintln!("Failed to send request: {}", e);
            break;
        }

        // 接收服务器的响应
        let mut buffer = [0; 1024];
        let num_bytes = stream.read(&mut buffer).await?;
        let response = String::from_utf8(buffer[..num_bytes].to_vec()).unwrap();

        println!("Received response: {}", response);

        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
