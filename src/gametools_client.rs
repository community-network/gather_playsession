use crate::structs::manager::ServerSessionReturn;
use chrono::prelude::*;
use reqwest::Url;

pub async fn request_sessions(
    server_id: &str,
    last_item_time: DateTime<Utc>,
) -> Result<ServerSessionReturn, reqwest::Error> {
    let url = Url::parse_with_params(
        "https://api.gametools.network/manager/server_sessions/",
        &[
            ("serverid", server_id),
            (
                "start_datetime",
                &last_item_time.to_rfc3339_opts(SecondsFormat::Secs, true)[..],
            ),
            (
                "end_datetime",
                &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)[..],
            ),
        ],
    )
    .unwrap();
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<ServerSessionReturn>().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
