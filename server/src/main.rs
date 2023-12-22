use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use rust_rbrserver::server::RacingServer;
use protocol::httpapi::{UserLogin, UserAccess, RaceInfo, UserJoin, UserUpdate, MetaHeader, MetaRaceResult, RaceState, DataFormat, RaceQuery, MetaRaceData};
use protocol::httpapi::{API_VERSION_STRING, META_HEADER_LEN};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let server = Arc::new(Mutex::new(RacingServer::default()));
    let data_clone = server.clone();
    let mng_clone = server.clone();

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
            tokio::spawn(handle_data_stream(stream, data_clone.clone()));
        }
    });

    let mgr_task = tokio::spawn(async move {
        loop {
            let mut server = mng_clone.lock().await;
            server.do_self_check();
            tokio::time::sleep(tokio::time::Duration::from_micros(200)).await;
        }
    });

    println!("Http server listening on port 8080");
    println!("Data listener listening on port 9493");

    let _ = tokio::join!(http_server, data_task, mgr_task);
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
    println!("Received user query race list");

    let server = data.lock().await;
    if let Some(response) = server.get_raceroom_list() {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race list failed!")
    }
}

#[actix_web::get("/api/race/info")]
async fn handle_http_race_info(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceQuery>) -> HttpResponse {
    let query = body.into_inner();
    println!("Received user query race info: {:?}", query);

    let server = data.lock().await;
    if let Some(response) = server.get_raceroom_info(&query.name) {
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
        HttpResponse::NotAcceptable().body("Create race Failed!")
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
    println!("Received user leave race info: {:?}", info);

    let mut server = data.lock().await;
    if server.leave_raceroom(&info.token) {
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
    if server.update_player_state(&info.token, info.state) {
        HttpResponse::Ok().body("Update race player state successful!")
    } else {
        HttpResponse::NotAcceptable().body("Update race player state failed!")
    }
}

async fn handle_data_stream(stream: TcpStream, data: Arc<Mutex<RacingServer>>) {
    let mut recvbuf = vec![0u8; 1024];
    let mut remain = Vec::<u8>::new();
    let (mut reader, writer) = stream.into_split();
    let writer_clone = Arc::new(Mutex::new(writer));
    while let Ok(n) = reader.read(&mut recvbuf).await {
        if n == 0 {
            break;
        }

        // 处理接收的数据
        // 这里只是简单地将接收到的数据打印出来
        // println!("Received data: {:?}", &recvbuf[..n]);

        let buffer = [&remain[..], &recvbuf[..n]].concat();
        let datalen = buffer.len();
        let mut offset = 0 as usize;

        while offset + META_HEADER_LEN <= datalen {
            if datalen < META_HEADER_LEN {
                break;
            }

            let head: MetaHeader = bincode::deserialize(&buffer[offset..offset+META_HEADER_LEN]).unwrap();
            if datalen < head.length as usize + META_HEADER_LEN {
                break;
            }

            let pack_data = &buffer[offset+META_HEADER_LEN..offset+META_HEADER_LEN+head.length as usize];
            meta_message_handle(head.clone(), pack_data, data.clone(), writer_clone.clone()).await;
            offset += META_HEADER_LEN + head.length as usize;
        }
        remain = (&buffer[offset..]).to_vec();
    }
}

async fn meta_message_handle(head: MetaHeader, pack_data: &[u8], data: Arc<Mutex<RacingServer>>, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut server = data.lock().await;
    match head.format {
        DataFormat::FmtUpdateState => { // race update game state
            let state: UserUpdate = bincode::deserialize(pack_data).unwrap();
            server.update_player_state(&state.token, state.state.clone());
            match state.state {
                RaceState::RaceReady => {
                    // wait player state ready, once ready send back to auto load game.
                    tokio::spawn(wait_all_players_ready(state.token, data.clone(), writer.clone()));
                }
                RaceState::RaceLoaded => {
                    // wait all player state loaded. once ready send back to start racing.
                    tokio::spawn(wait_all_players_loaded(state.token.clone(), data.clone(), writer.clone()));
                }
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    // wait all player state finish or retire, once ready send back race result to show.
                    tokio::spawn(wait_all_players_finish(state.token, data.clone(), writer.clone()));
                }
                _ => {}
            }
        }

        DataFormat::FmtUploadData => { // user exchange racing data.
            let racedata: MetaRaceData = bincode::deserialize(pack_data).unwrap();
            server.update_player_race_data(racedata);
        }
        _ => {}
    }
}

async fn wait_all_players_ready(token: String, server: Arc<Mutex<RacingServer>>, writer: Arc<Mutex<OwnedWriteHalf>>) {
    loop {
        let mut server = server.lock().await;
        if let Some(players) = server.get_room_all_players(&token) {
            if players.iter().all(|x| {
                match x.state {
                    RaceState::RaceReady => true,
                    _ => false,
                }
            }) {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_micros(500)).await;
    }

    let body = bincode::serialize(&UserUpdate{token: token, state: RaceState::RaceLoad}).unwrap();
    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtRaceCommand}).unwrap();
    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
}

async fn wait_all_players_loaded(token: String, server: Arc<Mutex<RacingServer>>, writer: Arc<Mutex<OwnedWriteHalf>>) {
    loop {
        let mut server = server.lock().await;
        if let Some(players) = server.get_room_all_players(&token) {
            if players.iter().all(|x| {
                match x.state {
                    RaceState::RaceLoaded => true,
                    _ => false,
                }
            }) {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    let body = bincode::serialize(&UserUpdate{token: token, state: RaceState::RaceStart}).unwrap();
    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtRaceCommand}).unwrap();
    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
}

async fn wait_all_players_finish(token: String, server: Arc<Mutex<RacingServer>>, writer: Arc<Mutex<OwnedWriteHalf>>) {
    loop {
        let mut server = server.lock().await;
        if let Some(mut players) = server.get_room_all_players(&token) {
            if players.clone().iter().all(|x| {
                match x.state {
                    RaceState::RaceRetired => true,
                    RaceState::RaceFinished => true,
                    _ => false,
                }
            }) {
                break;
            }

            let mut response = MetaRaceResult::default();
            players.sort_by(|a, b| a.sort_by_time(b));
            for player in players {
                response.board.push(player.race_data);
            }
            let body = bincode::serialize(&response).unwrap();
            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceData}).unwrap();
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // send final result to show.
    let mut server = server.lock().await;
    if let Some(mut players) = server.get_room_all_players(&token) {
        let mut response = MetaRaceResult::default();
        players.sort_by(|a, b| a.sort_by_time(b));
        for player in players {
            response.board.push(player.race_data);
        }

        let body = bincode::serialize(&response).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceResult}).unwrap();
        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
    }
}