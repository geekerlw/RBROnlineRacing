use std::path::PathBuf;
use rbnproto::{httpapi::UserScore, metaapi::MetaRaceResult};
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

    fn get_license(&mut self, score: i32) -> String {
        let mut license = String::from("Rookie");
        if score < 500 {
            license = "Rookie".to_string();
        } else if 500 <= score && score < 1000 {
            license = "Amateur".to_string();
        } else if 1000 <= score && score < 1500 {
            license = "Master".to_string();
        } else if 1500 <= score && score < 2000 {
            license = "Profession".to_string();
        } else if 2000 <= score {
            license = "Buglike".to_string();
        }

        license
    }

    pub async fn connect(&mut self) -> SqlitePool {
        SqlitePool::connect(self.dbfile.to_str().unwrap()).await.unwrap()
    }

    pub async fn query_user_score(&mut self, player: &LobbyPlayer) -> Option<UserScore> {
        let conn = self.connect().await;
        let user: Option<User> = sqlx::query_as("SELECT * FROM user WHERE name = ?")
        .bind(&player.profile_name)
        .fetch_optional(&conn)
        .await.unwrap_or_default();

        if let Some(user) = user {
            return Some(UserScore { name: user.name.clone(), license: user.license.clone(), score: user.score.clone() });
        }

        None
    }

    pub async fn query_all_user_score(&mut self) -> Vec<UserScore> {
        let conn = self.connect().await;
        let users: Option<Vec<User>> = sqlx::query_as::<_, User>("SELECT * FROM user order by score desc")
        .fetch_all(&conn)
        .await.ok();

        let mut result = vec![];
        if let Some(users) = users {
            for user in users {
                result.push(UserScore { name: user.name.clone(), license: user.license.clone(), score: user.score.clone() });
            }
        }

        result
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
                let new_license = self.get_license(new_score);

                sqlx::query("UPDATE user SET license = ?, score = ? where id = ?")
                .bind(new_license)
                .bind(new_score)
                .bind(&user.id)
                .execute(&conn)
                .await.unwrap_or_default();
            }
        }
    }
}