use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use server::server::RacingServer;
use protocol::httpapi::{UserLogin, UserAccess, RaceInfo, UserJoin, UserUpdate, MetaHeader};
use protocol::httpapi::API_VERSION_STRING;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let server = Arc::new(Mutex::new(RacingServer::default()));
    let data_clone = server.clone();

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
        .service(handle_http_race_leave)
        .service(handle_http_race_update_state)
    })
    .bind("127.0.0.1:8080")?
    .run();

    let data_task = tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:9493").await.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let socket = Arc::new(Mutex::new(stream));
            tokio::spawn(handle_data_stream(socket, data_clone));
        }
    });

    println!("Http server listening on port 8080");
    println!("Data listener listening on port 9493");

    let _ = tokio::join!(http_server, data_task);
    Ok(())
}

#[actix_web::get("/api/version")]
async fn handle_http_api_version() -> HttpResponse {
    HttpResponse::Ok().body(API_VERSION_STRING)
}

#[actix_web::post("api/user/login")]
async fn handle_http_user_login(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserLogin>) -> HttpResponse {
    let user = body.into_inner();
    println!("Received user login: {:?}", user);

    let token = Uuid::new_v4();
    let response = token.to_string();
    let mut server = data.lock().await;

    if server.player_login(user, token) {
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
        HttpResponse::NotAcceptable().body("Logout failed!")
    }
}

#[actix_web::get("/api/race/list")]
async fn handle_http_race_list(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    let server = data.lock().await;
    if let Some(response) = server.get_raceroom_list() {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race list failed!")
    }
}

#[actix_web::get("/api/race/info")]
async fn handle_http_race_info(data: web::Data<Arc<Mutex<RacingServer>>>, name: web::Query<String>) -> HttpResponse {
    let server = data.lock().await;
    if let Some(response) = server.get_raceroom_info(&name) {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race info failed!")
    }
}

#[actix_web::post("/api/race/create")]
async fn handle_http_race_create(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceInfo>) -> HttpResponse {
    let info = body.into_inner();
    println!("Received user create race info: {:?}", info);

    let mut server = data.lock().await;
    if server.create_raceroom(info) {
        HttpResponse::Ok().body("Create race successful!")
    } else {
        HttpResponse::Ok().body("Create race Failed!")
    }
}

#[actix_web::post("/api/race/join")]
async fn handle_http_race_join(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserJoin>) -> HttpResponse {
    let info = body.into_inner();
    println!("Received user join race info: {:?}", info);

    let mut server = data.lock().await;
    if server.join_raceroom(info.room, info.token) {
        HttpResponse::Ok().body("Join race successful!")
    } else {
        HttpResponse::NotFound().body("Join race failed!")
    }
}

#[actix_web::post("/api/race/leave")]
async fn handle_http_race_leave(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserAccess>) -> HttpResponse {
    let info: UserAccess = body.into_inner();
    println!("Received user logout: {:?}", info);

    let mut server = data.lock().await;
    if server.leave_raceroom(info.token) {
        HttpResponse::Ok().body("Leave race room successful!")
    } else {
        HttpResponse::NotAcceptable().body("Leave race room failed!")
    }
}

#[actix_web::put("/api/race/state")]
async fn handle_http_race_update_state(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserUpdate>) -> HttpResponse {
    let info: UserUpdate = body.into_inner();
    println!("Received user logout: {:?}", info);

    let mut server = data.lock().await;
    if server.update_player_state(info.token, info.state) {
        HttpResponse::Ok().body("Update race player state successful!")
    } else {
        HttpResponse::NotAcceptable().body("Update race player state failed!")
    }
}

async fn handle_data_stream(stream: Arc<Mutex<TcpStream>>, data: Arc<Mutex<RacingServer>>) {
    let mut recvbuf = vec![0u8; 1024];
    let mut remain = Vec::<u8>::new();
    while let Ok(n) = stream.lock().await.read(&mut recvbuf).await {
        if n == 0 {
            break;
        }

        // 处理接收的数据
        // 这里只是简单地将接收到的数据打印出来
        println!("Received data: {:?}", &recvbuf[..n]);

        let buffer = [&remain[..], &recvbuf[..]].concat();
        let datalen = buffer.len();

        if datalen <= 4 {
            remain = buffer.to_vec();
            continue;
        }

        let head: MetaHeader = bincode::deserialize(&buffer[..4]).unwrap();
        if datalen <= head.length as usize + 4 {
            remain = buffer.to_vec();
            continue;
        }

        let mut server = data.lock().await;
        let pack_data = &buffer[4..4+head.length as usize];
        match head.format {
            1 => { // user meta data socket login.
                let access: UserAccess = bincode::deserialize(pack_data).unwrap();
                if !server.meta_player_login(access) {
                    break; // can't access, disconnect.
                }

                // tokio::spawn to wait player state ready, once ready send 1 back to auto load game.
            }
            
            2 => { // race load game over and pause in game start state.
                // tokio::spawn to wait all player state loaded. once ready send 2 back to start racing.
            }

            3 => { // race running, loop update player's state.
                // tokio::spawn to calc room's players racedata, sort and send back the leader board to show.
            }

            4 => { // player finish or retire the race.
                // tokio::spawn to wait all player state finish or retire, once ready send back race result to show.

                // once send out the result, break and auto close the socket.
            }

            10 => { // user exchange racing data.
                let racedata = bincode::deserialize(pack_data).unwrap();
                if let Some(result) = server.meta_player_exchange_race_data(racedata) {
                    let bodybuf = bincode::serialize(&result).unwrap();
                    let head = MetaHeader {length: bodybuf.len() as u16, format: 0x8000 | 10u16};
                    let headbuf = bincode::serialize(&head).unwrap();
                    let data = [&headbuf[..], &bodybuf[..]].concat();
                    stream.lock().await.write_all(&data).await.unwrap();
                }
            }
            _ => {
                break; //data type error, auto close.
            }
        }

        remain = (&buffer[4 + head.length as usize..]).to_vec();
    }
}

async fn handle_data_service(stream: Arc<Mutex<TcpStream>>, data: Arc<Mutex<RacingServer>>) {
    while let Some(msg) = rx.recv().await {
        stream.lock().await.write_all(msg.as_bytes()).await.unwrap();
    }
}