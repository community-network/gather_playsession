use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerSessionReturn {
    pub data: Vec<ServerSession>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerSession {
    pub stats: SessionStats,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub game_name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionStats {
    #[serde(rename = "timePlayed")]
    pub time_played: u64,
    pub wins: u64,
    pub losses: u64,
    pub kills: u64,
    pub deaths: u64,
    pub kits: Vec<Kits>,
    pub score: u64,
    pub gamemodes: Vec<Gamemodes>,
    #[serde(rename = "killDeath")]
    pub kill_death: f64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Gamemodes {
    pub name: String,
    pub score: f64,
    pub wins: u64,
    pub losses: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Kits {
    pub name: String,
    pub score: f64,
    pub kills: f64,
    #[serde(rename = "timePlayed")]
    pub time_played: f64,
}
