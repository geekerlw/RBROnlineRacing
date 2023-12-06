use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;

use server::player::RacePlayer;
use server::server::RacingServer;
use protocol::httpapi::{UserLogin, UserAccess};
use protocol::httpapi::API_VERSION_STRING;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let server = Arc::new(Mutex::new(RacingServer::default()));
    let server_clone = server.clone();

    let http_server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(server.clone()))
        .service(handle_http_api_version)
        .service(handle_http_user_login)
        .service(handle_http_user_logout)
        .service(handle_http_race_list)
        .service(handle_http_race_info)
        .service(handle_http_race_create)
        .service(handle_http_race_join)
        .service(handle_http_race_exit)
        .service(handle_http_race_update_state)
        .service(handle_http_race_get_result)
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

#[actix_web::get("/api/version")]
async fn handle_http_api_version() -> HttpResponse {
    HttpResponse::Ok().body(API_VERSION_STRING)
}

#[actix_web::post("api/user/login")]
async fn handle_http_user_login(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserLogin>) -> HttpResponse {
    let user = body.into_inner();
    println!("Received user login: {:?}", user);

    if user.passwd == "simrallycn" {
        let player = RacePlayer::new(user.name);
        let mut server = data.lock().await;
        let response = player.user_token.to_string();
        server.player_login(player);
        HttpResponse::Ok().body(response)
    } else {
        HttpResponse::Unauthorized().body("Login failed!")
    }
}

#[actix_web::post("/api/user/logout")]
async fn handle_http_user_logout(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserAccess>) -> HttpResponse {
    let user: UserAccess = body.into_inner();
    println!("Received user logout: {:?}", user);

    let mut server = data.lock().await;
    if server.player_logout(user.token) {
        HttpResponse::Ok().body("Logout successful!")
    } else {
        HttpResponse::Unauthorized().body("Logout failed!")
    }
}

#[actix_web::get("/api/race/list")]
async fn handle_http_race_list() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::get("/api/race/info")]
async fn handle_http_race_info() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::get("/api/race/create")]
async fn handle_http_race_create() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::post("/api/race/join/{token}")]
async fn handle_http_race_join() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::post("/api/race/exit/{token}")]
async fn handle_http_race_exit() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::put("/api/race/state")]
async fn handle_http_race_update_state() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}

#[actix_web::get("/api/race/result/")]
async fn handle_http_race_get_result() -> HttpResponse {
    HttpResponse::Ok().body("not support now.")
}