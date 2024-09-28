// @generated automatically by Diesel CLI.

diesel::table! {
    scoreboard (id) {
        player_id -> Int8,
        server_id -> Text,
        time_stamp -> Timestamptz,
        id -> Text,
        time_played -> Int8,
        wins -> Int8,
        losses -> Int8,
        kills -> Int8,
        deaths -> Int8,
        score -> Int8,
        name -> Text,
    }
}
