use std::error::Error;
mod db;
mod gametools_client;
mod structs;
use std::{
    env,
    ops::Add,
    sync::{atomic, Arc},
    time::Duration,
};
use tokio::time::sleep;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mins_between_runs = 5;
    let last_update = Arc::new(atomic::AtomicI64::new(chrono::Utc::now().timestamp() / 60));
    let last_update_clone = Arc::clone(&last_update);

    flexi_logger::Logger::try_with_str("info")?.start()?;
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => log::info!(".env not found, using env variables..."),
    };
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_id = env::var("SERVER_ID").expect("SERVER_ID must be set");
    let keepalive_port = env::var("KEEPALIVE_PORT")
        .expect("KEEPALIVE_PORT must be set")
        .parse()
        .expect("KEEPALIVE_PORT must be a valid port number");

    log::info!("Starting...");

    let connection = &mut db::client::PgClient::connect(database_url)?;

    tokio::spawn(async move {
        let hello = warp::any().map(move || {
            let last_update_i64 = last_update_clone.load(atomic::Ordering::Relaxed);
            let now_minutes = chrono::Utc::now().timestamp() / 60;

            // error if 10 minutes without updates
            if (now_minutes - last_update_i64) > 10 {
                warp::reply::with_status(
                    format!("{}", now_minutes - last_update_i64),
                    warp::http::StatusCode::SERVICE_UNAVAILABLE,
                )
            } else {
                warp::reply::with_status(
                    format!("{}", now_minutes - last_update_i64),
                    warp::http::StatusCode::OK,
                )
            }
        });
        warp::serve(hello).run(([0, 0, 0, 0], keepalive_port)).await;
    });

    // run on startup by removing mins_between_runs
    let mut last_ran = chrono::Utc::now() - chrono::Duration::minutes(mins_between_runs);
    log::info!("Started");

    loop {
        let run = last_ran.add(chrono::Duration::minutes(mins_between_runs)) <= chrono::Utc::now();
        if run {
            log::info!("Starting new run");
            last_ran = chrono::Utc::now();

            let last_item_time = connection.get_last_timestamp()?;
            let sessions =
                gametools_client::request_sessions(&server_id[..], last_item_time).await?;

            let mut new_sessions = 0;
            for item in sessions.data {
                let play_session = std::convert::Into::<db::models::Scoreboard>::into(item);
                match connection.create_playsession(play_session) {
                    Ok(_) => new_sessions += 1,
                    Err(diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _, // skipping the requested last_item_time item, since it already exists
                    )) => {}
                    Err(e) => log::error!("{}", e),
                };
            }
            log::info!("{} new playsessions added", new_sessions);
            last_update.store(
                chrono::Utc::now().timestamp() / 60,
                atomic::Ordering::Relaxed,
            );
        } else {
            let mins = last_ran + chrono::Duration::minutes(mins_between_runs);
            log::info!(
                "Waiting {:#?} minutes before next run",
                (mins - last_ran).num_minutes()
            );
            sleep(Duration::from_secs(60)).await;
        }
    }
}
