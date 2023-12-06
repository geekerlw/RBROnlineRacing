use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;

use server::server::RacingServer;

async fn handle_http_request(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    let mut server = data.lock().await;
    server.count += 1;
    HttpResponse::Ok().body(server.count.to_string())
}

async fn handle_data_stream(mut stream: TcpStream, data: Arc<Mutex<RacingServer>>) {
    let mut buffer = vec![0u8; 1024];
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        // 处理接收的数据
        // 这里只是简单地将接收到的数据打印出来
        println!("Received data: {:?}", &buffer[..n]);

        let mut response = String::from("response from server, count: ");
        let server = data.lock().await;
        response.push_str(server.count.to_string().as_str());
        stream.write_all(response.as_bytes()).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let server = Arc::new(Mutex::new(RacingServer::default()));
    let server_clone = server.clone();

    let http_server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(server.clone()))
        .route("/", web::get().to(handle_http_request))
    })
    .bind("127.0.0.1:8080")?
    .run();

    let data_task = tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:9493").await.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let data = server_clone.clone();
            tokio::spawn(handle_data_stream(stream, data));
        }
    });

    println!("Http server listening on port 8080");
    println!("Data listener listening on port 9493");

    let _ = tokio::join!(http_server, data_task);
    Ok(())
}