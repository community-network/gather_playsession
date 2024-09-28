use crate::db::*;
use crate::structs::manager::ServerSession;
use chrono::{prelude::*, DateTime, Utc};
use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, Debug)]
#[diesel(table_name = schema::scoreboard)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Scoreboard {
    pub player_id: i64,
    pub server_id: String,
    pub time_stamp: DateTime<Utc>,
    pub id: String,
    pub time_played: i64,
    pub wins: i64,
    pub losses: i64,
    pub kills: i64,
    pub deaths: i64,
    pub score: i64,
    pub name: String,
}

impl From<ServerSession> for Scoreboard {
    fn from(session: ServerSession) -> Self {
        let stats = session.stats;
        let timestamp = Utc.from_utc_datetime(
            &NaiveDateTime::parse_from_str(&session.time_stamp[..], "%Y-%m-%d %H:%M:%S%.6f")
                .unwrap(),
        ); // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        Scoreboard {
            player_id: session.player_id,
            server_id: session.server_id,
            time_stamp: timestamp,
            id: session.id,
            time_played: stats.time_played as i64,
            wins: stats.wins as i64,
            losses: stats.losses as i64,
            kills: stats.kills as i64,
            deaths: stats.deaths as i64,
            score: stats.score as i64,
            name: stats.name,
        }
    }
}
