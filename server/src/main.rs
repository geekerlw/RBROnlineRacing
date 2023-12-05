use server::server::RacingServer;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let mut server = RacingServer::default();
    //server.spawn_http_task();
    let data_task = server.spawn_data_task();

    tokio::join!(data_task);

    Ok(())
}