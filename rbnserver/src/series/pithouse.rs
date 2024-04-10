use crate::player::RacePlayer;


#[derive(Default)]
pub struct RacePitHouse {
    pub players: Vec<RacePlayer>,
    limit: Option<usize>,
    passwd: Option<String>,
}

#[allow(dead_code)]
impl RacePitHouse {
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn set_pass(&mut self, pass: String) {
        self.passwd = Some(pass);
    }

    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, tokenstr: &String) {
        self.players.retain(|x| &x.tokenstr != tokenstr);
    }

    pub fn get_player(&mut self, tokenstr: &String) -> Option<&mut RacePlayer> {
        for (_, player) in self.players.iter_mut().enumerate() {
            if &player.tokenstr == tokenstr {
                return Some(player);
            }
        }
        None
    }

    pub fn is_player_exist(&mut self, name: &String) -> bool {
        for player in &self.players {
            if &player.profile_name == name {
                return true;
            }
        }
        return false;
    }

    pub fn is_empty(&mut self) -> bool {
        self.players.is_empty()
    }

    pub fn is_full(&mut self) -> bool {
        if let Some(limit) = self.limit {
            return self.players.len() >= limit;
        }
        false
    }

    pub fn is_locked(&mut self) -> bool {
        if let Some(_) = self.passwd {
            return true;
        }
        false
    }

    pub fn pass_match(&mut self, passwd: &String) -> bool {
        if let Some(pass) = &self.passwd {
            return passwd == pass;
        }
        return false;
    }

    pub fn notify_all_players_race_notice(&mut self, notice: String) {
        if self.is_empty() {
            return;
        }

        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_racenotice(&notice).await;
            }
        });
    }
}