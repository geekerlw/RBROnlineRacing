use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Local;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, trace};

use crate::server::RacingServer;
use rbnproto::httpapi::{RaceConfigUpdate, RaceCreate, RaceInfoUpdate, RaceQuery, UserHeart, UserLogin, UserLogout, UserQuery};
use rbnproto::API_VERSION_STRING;
use rbnproto::metaapi::{META_HEADER_LEN, RaceUpdate, RaceAccess, RaceJoin, RaceLeave, MetaHeader, DataFormat, MetaRaceData};

mod db;
mod series;
mod lobby;
mod player;
mod server;

/// Set http and metadata ports.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port of the http service port
    #[arg(short, long, default_value_t = 23555)]
    port: u16,

    /// Port of the meta data service port
    #[arg(short, long, default_value_t = 23556)]
    data: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    env_logger::init();

    let args = Args::parse();
    let http_addr = "0.0.0.0:".to_string() + &args.port.to_string();
    let meta_addr = "0.0.0.0:".to_string() + &args.data.to_string();

    let server = Arc::new(Mutex::new(RacingServer::default().init()));
    let data_clone = server.clone();
    let mng_clone = server.clone();
    db::RaceDB::default().migrate().await;

    let http_server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(server.clone()))
        .service(handle_http_api_version)
        .service(handle_http_user_login)
        .service(handle_http_user_heartbeat)
        .service(handle_http_user_logout)
        .service(handle_http_user_fetch_score)
        .service(handle_http_race_fetch_news)
        .service(handle_http_race_fetch_list)
        .service(handle_http_race_get_info)
        .service(handle_http_race_update_info)
        .service(handle_http_race_get_state)
        .service(handle_http_race_update_state)
        .service(handle_http_player_get_config)
        .service(handle_http_player_update_config)
        .service(handle_http_race_get_start)
        .service(handle_http_race_set_start)
        .service(handle_http_race_create)
        .service(handle_http_race_join)
        .service(handle_http_race_leave)
        .service(handle_http_race_destroy)
        .service(handle_http_image_get)
        .service(handle_http_file_download)
        .service(handle_web_index)
        .service(handle_web_rankboard)
    })
    .bind(http_addr)?
    .run();

    let data_task = tokio::spawn(async move {
        let listener = TcpListener::bind(meta_addr).await.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_data_stream(stream, data_clone.clone()));
        }
    });

    let mgr_task = tokio::spawn(async move {
        loop {
            let mut server = mng_clone.lock().await;
            server.recycle_invalid_races();
            server.recycle_invalid_players();
            for (_, race) in server.races.iter_mut() {
                race.framed_schedule();
            }
            drop(server);
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });

    info!("Rust RBR Online Server Version: V{}", std::env!("CARGO_PKG_VERSION"));
    info!("Http server listening on port {}", args.port);
    info!("Data listener listening on port {}", args.data);

    let _ = tokio::join!(http_server, data_task, mgr_task);
    Ok(())
}

#[actix_web::get("/api/version")]
async fn handle_http_api_version() -> HttpResponse {
    HttpResponse::Ok().body(API_VERSION_STRING)
}

#[actix_web::post("/api/user/login")]
async fn handle_http_user_login(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserLogin>) -> HttpResponse {
    let user = body.into_inner();
    info!("Received user login: {:?}", user);

    let mut server = data.lock().await;
    if let Some(tokenstr) = server.user_login(user).await {
        HttpResponse::Ok().body(tokenstr)
    } else {
        HttpResponse::Unauthorized().body("Login failed!")
    }
}

#[actix_web::post("/api/user/heartbeat")]
async fn handle_http_user_heartbeat(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserHeart>) -> HttpResponse {
    let user = body.into_inner();
    trace!("Received user heartbeat: {:?}", user);
    let mut server = data.lock().await;
    server.user_heartbeat(user);

    HttpResponse::Ok().body(Local::now().to_string())
}

#[actix_web::post("/api/user/logout")]
async fn handle_http_user_logout(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserLogout>) -> HttpResponse {
    let user: UserLogout = body.into_inner();
    info!("Received user logout: {:?}", user);

    let mut server = data.lock().await;
    if server.user_logout(user) {
        HttpResponse::Ok().body("Logout successful!")
    } else {
        HttpResponse::NotAcceptable().body("Logout failed!")
    }
}

#[actix_web::get("/api/user/score")]
async fn handle_http_user_fetch_score(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserQuery>) -> HttpResponse {
    let query = body.into_inner();
    trace!("Received user query user score: {:?}", query);

    let mut server = data.lock().await;
    if let Some(response) = server.get_user_score(&query.token).await {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race info failed!")
    }
}

#[actix_web::get("/api/race/news")]
async fn handle_http_race_fetch_news(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    trace!("Received user query race news");

    let mut server = data.lock().await;
    HttpResponse::Ok().body(serde_json::to_string(&server.get_race_news()).unwrap())
}

#[actix_web::get("/api/race/list")]
async fn handle_http_race_fetch_list(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    trace!("Received user query race list");

    let mut server = data.lock().await;
    if let Some(response) = server.get_race_list() {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race list failed!")
    }
}

#[actix_web::get("/api/race/info")]
async fn handle_http_race_get_info(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceQuery>) -> HttpResponse {
    let query = body.into_inner();
    trace!("Received user query race info: {:?}", query);

    let mut server = data.lock().await;
    if let Some(response) = server.get_race_info(&query.name) {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race info failed!")
    }
}

#[actix_web::put("/api/race/info")]
async fn handle_http_race_update_info(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceInfoUpdate>) -> HttpResponse {
    let update = body.into_inner();
    trace!("Received user update race info: {:?}", update);

    let mut server = data.lock().await;
    if server.update_race_info(update) {
        HttpResponse::Ok().body("Update race info successful!")
    } else {
        HttpResponse::NoContent().body("Update Race info failed!")
    }
}

#[actix_web::get("/api/player/config")]
async fn handle_http_player_get_config(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<UserQuery>) -> HttpResponse {
    let query = body.into_inner();
    trace!("Received user query race config: {:?}", query);

    let mut server = data.lock().await;
    if let Some(response) = server.get_player_race_config(&query) {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race config failed!")
    }
}

#[actix_web::put("/api/player/config")]
async fn handle_http_player_update_config(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceConfigUpdate>) -> HttpResponse {
    let update = body.into_inner();
    trace!("Received user update race config: {:?}", update);

    let mut server = data.lock().await;
    if server.update_player_race_config(update) {
        HttpResponse::Ok().body("Update race config successful!")
    } else {
        HttpResponse::NoContent().body("Update Race config failed!")
    }
}

#[actix_web::get("/api/race/state")]
async fn handle_http_race_get_state(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceQuery>) -> HttpResponse {
    let query = body.into_inner();
    trace!("Received user query race users state: {:?}", query);

    let mut server = data.lock().await;
    if let Some(response) = server.get_race_userstate(&query.name) {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race user state failed!")
    }
}

#[actix_web::put("/api/race/state")]
async fn handle_http_race_update_state(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceUpdate>) -> HttpResponse {
    let info: RaceUpdate = body.into_inner();
    info!("Received user update race state: {:?}", info);

    let mut server = data.lock().await;
    if server.update_player_state(&info) {
        HttpResponse::Ok().body("Update race player state successful!")
    } else {
        HttpResponse::NotAcceptable().body("Update race player state failed!")
    }
}

#[actix_web::get("/api/race/start")]
async fn handle_http_race_get_start(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceQuery>) -> HttpResponse {
    let query = body.into_inner();
    trace!("Received user query room race start state: {:?}", query);

    let mut server = data.lock().await;
    if let Some(response) = server.get_race_started(&query.name) {
        HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
    } else {
        HttpResponse::NoContent().body("Get Race started state failed!")
    }
}

#[actix_web::put("/api/race/start")]
async fn handle_http_race_set_start(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceAccess>) -> HttpResponse {
    let access = body.into_inner();
    trace!("Received user set room race start: {:?}", access);

    let mut server = data.lock().await;
    if server.set_race_started(&access) {
        HttpResponse::Ok().body("Update Race started state successful!")
    } else {
        HttpResponse::NoContent().body("Update Race started state failed!")
    }
}

#[actix_web::post("/api/race/create")]
async fn handle_http_race_create(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceCreate>) -> HttpResponse {
    let info = body.into_inner();
    info!("Received user create race info: {:?}", info);

    let mut server = data.lock().await;
    if server.create_race(info) {
        HttpResponse::Ok().body("Create race successful!")
    } else {
        HttpResponse::NotAcceptable().body("Create race Failed!")
    }
}

#[actix_web::post("/api/race/join")]
async fn handle_http_race_join(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceJoin>) -> HttpResponse {
    let info = body.into_inner();
    info!("Received user join race info: {:?}", info);

    let mut server = data.lock().await;
    if server.join_race(info) {
        HttpResponse::Ok().body("Join race successful!")
    } else {
        HttpResponse::NotFound().body("Join race failed!")
    }
}

#[actix_web::post("/api/race/leave")]
async fn handle_http_race_leave(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceLeave>) -> HttpResponse {
    let info: RaceLeave = body.into_inner();
    info!("Received user leave race info: {:?}", info);

    let mut server = data.lock().await;
    if server.leave_race(info.room, info.token) {
        HttpResponse::Ok().body("Leave race room successful!")
    } else {
        HttpResponse::NotAcceptable().body("Leave race room failed!")
    }
}

#[actix_web::post("/api/race/destroy")]
async fn handle_http_race_destroy(data: web::Data<Arc<Mutex<RacingServer>>>, body: web::Json<RaceAccess>) -> HttpResponse {
    let info: RaceAccess = body.into_inner();
    info!("Received user destroy race info: {:?}", info);

    let mut server = data.lock().await;
    if server.destroy_race(info.room, info.token) {
        HttpResponse::Ok().body("Destroy race room successful!")
    } else {
        HttpResponse::NotAcceptable().body("Destroy race room failed!")
    }
}

#[actix_web::get("/api/image/{file}")]
async fn handle_http_image_get(data: web::Data<Arc<Mutex<RacingServer>>>, path: web::Path<String>) -> HttpResponse {
    let image_file = path.into_inner();
    let mut server = data.lock().await;
    if let Some(image_data) = server.load_image(&image_file).await {
        return HttpResponse::Ok().content_type("image/png").body(image_data);
    }
    return HttpResponse::NoContent().body("No such file.");
}

#[actix_web::get("/api/download/{file}")]
async fn handle_http_file_download(data: web::Data<Arc<Mutex<RacingServer>>>, path: web::Path<String>) -> HttpResponse {
    let download_file = path.into_inner();
    let mut server = data.lock().await;
    if let Some(file_data) = server.load_file(&download_file).await {
        return HttpResponse::Ok().content_type("application/zip")
        .append_header(("Content-Disposition", format!("attachment; filename={}", download_file)))
        .body(file_data);
    }
    return HttpResponse::NoContent().body("No such file.");
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
        // trace!("Received data: {:?}", &recvbuf[..n]);

        let buffer = [&remain[..], &recvbuf[..n]].concat();
        let datalen = buffer.len();
        let mut offset = 0 as usize;

        while offset + META_HEADER_LEN <= datalen {
            if datalen < META_HEADER_LEN {
                break;
            }
            let head: MetaHeader = bincode::deserialize(&buffer[offset..offset+META_HEADER_LEN]).unwrap();

            if (offset + META_HEADER_LEN + head.length as usize) > datalen {
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
        DataFormat::FmtUserAccess => {
            let user: RaceAccess = bincode::deserialize(pack_data).unwrap();
            info!("recv racer access: {:?}", user);
            server.race_player_access(&user, writer);
        }
        DataFormat::FmtUpdateState => { // race update game state
            let state: RaceUpdate = bincode::deserialize(pack_data).unwrap();
            info!("recv racer state update: {:?}", state);
            server.update_player_state(&state);
        }

        DataFormat::FmtUploadData => { // user exchange racing data.
            let racedata: MetaRaceData = bincode::deserialize(pack_data).unwrap();
            server.update_player_race_data(racedata);
        }
        _ => {}
    }
}

#[actix_web::get("/")]
async fn handle_web_index(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    let server = data.lock().await;
    let mut context = tera::Context::new();
    context.insert("last_release", &format!("RBNHelper_{}.zip", std::env!("CARGO_PKG_VERSION")));
    let rendered = server.tera.render("index.html", &context)
        .expect("failed to render template");

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::get("/rankboard")]
async fn handle_web_rankboard(data: web::Data<Arc<Mutex<RacingServer>>>) -> HttpResponse {
    let mut server = data.lock().await;

    let mut context = tera::Context::new();
    let players = server.get_all_user_score().await;
    context.insert("players", &players);

    let rendered = server.tera.render("rank.html", &context)
        .expect("Failed to render template");

    HttpResponse::Ok().content_type("text/html").body(rendered)
}