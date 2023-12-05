use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use std::net::SocketAddr;

use crate::lobby::RaceLobby;
use crate::room::RaceRoom;

#[derive(Default)]
pub struct RacingServer {
    lobby: RaceLobby,
    room: RaceRoom,
}

impl RacingServer {
    // pub async fn spawn_http_task(&mut self) -> std::io::Result<()> {
    //     tokio::spawn(async move {
    //         let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    //         HttpServer::new(|| {
    //             App::new().route("/", web::get().to(Self::handle_http_request))
    //         })
    //         .bind(addr)?
    //         .run()
    //         .await
    //     });

    //     Ok(())
    // }

    pub async fn spawn_data_task(&mut self) -> JoinHandle<()> {
        let task = tokio::spawn(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
            let listener = TcpListener::bind(addr).await.unwrap();
            println!("Data listener listening on port 8888");

            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(Self::handle_data_stream(stream));
            }
        });

        task
    }

    async fn handle_http_request(&mut self) -> HttpResponse {
        HttpResponse::Ok().body("Hello, World!")
    }

    async fn handle_data_stream(mut stream: TcpStream) {
        let mut buffer = vec![0u8; 1024];
        while let Ok(n) = stream.read(&mut buffer).await {
            if n == 0 {
                break;
            }

            // 处理接收的数据
            // 这里只是简单地将接收到的数据打印出来
            println!("Received data: {:?}", &buffer[..n]);

            // 回复数据
            let response = b"Response from server";
            stream.write_all(response).await.unwrap();
        }
    }

}