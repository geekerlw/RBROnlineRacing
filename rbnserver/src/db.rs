use std::path::PathBuf;
use rbnproto::metaapi::MetaRaceResult;
use sqlx::SqlitePool;

use crate::player::LobbyPlayer;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    passwd: String,
    license: String,
    score: i32,
}

pub struct RaceDB {
    dbfile: PathBuf,
}

impl Default for RaceDB {
    fn default() -> Self {
        let file = std::env::current_exe().unwrap().parent().unwrap().join("rbndata.db");
        Self { 
            dbfile: file,
        }
    }
}

impl RaceDB {
    pub async fn migrate(&mut self) {
        if !self.dbfile.exists() {
            std::fs::File::create(self.dbfile.clone()).expect("Failed to create database file.");
        }
    
        let pool = SqlitePool::connect(self.dbfile.to_str().unwrap()).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
    }

    pub async fn connect(&mut self) -> SqlitePool {
        SqlitePool::connect(self.dbfile.to_str().unwrap()).await.unwrap()
    }

    pub async fn on_user_login(&mut self, player: &LobbyPlayer) {
        let conn = self.connect().await;
        let user: Option<User> = sqlx::query_as("SELECT * FROM user WHERE name = ?")
        .bind(&player.profile_name)
        .fetch_optional(&conn)
        .await.unwrap_or_default();

        if user.is_none() {
            sqlx::query("INSERT INTO user (name) VALUES (?)")
            .bind(&player.profile_name)
            .execute(&conn)
            .await.unwrap_or_default();
        }
    }

    pub async fn on_race_finished(&mut self, results: &Vec<MetaRaceResult>) {
        let conn = self.connect().await;
        for result in results {
            let user: Option<User> = sqlx::query_as("SELECT * FROM user WHERE name = ?")
            .bind(&result.profile_name)
            .fetch_optional(&conn)
            .await.unwrap_or_default();

            if let Some(user) = user {
                let new_score = user.score + result.score;

                sqlx::query("update user SET score = ? where id = ?")
                .bind(new_score)
                .bind(&user.id)
                .execute(&conn)
                .await.unwrap_or_default();
            }
        }
    }
}