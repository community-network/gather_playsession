use crate::db::schema::scoreboard::dsl::*;
use chrono::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use diesel::prelude::*;
use diesel::result::Error::NotFound;
use std::error::Error;

pub struct PgClient {
    pub client: PgConnection,
}

impl PgClient {
    pub fn connect(database_url: String) -> Result<Self, Box<dyn Error>> {
        Ok(PgClient {
            client: PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        })
    }

    pub fn get_last_timestamp(&mut self) -> Result<DateTime<Utc>, diesel::result::Error> {
        let result = scoreboard
            .select(time_stamp)
            .order_by(time_stamp.desc())
            .first::<DateTime<Utc>>(&mut self.client);
        // if DB is empty, return 7 days ago, since thats how long it gets saved by gametools
        match result {
            Ok(date_time) => Ok(date_time),
            Err(e) if e == NotFound => Ok(Utc::now() - chrono::Duration::days(7)),
            Err(err) => Err(err),
        }
    }

    pub fn create_playsession(
        &mut self,
        item: crate::db::models::Scoreboard,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(crate::db::schema::scoreboard::table)
            .values(&item)
            .execute(&mut self.client)
    }
}
