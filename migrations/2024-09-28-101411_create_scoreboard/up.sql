-- Your SQL goes here https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html
CREATE TABLE scoreboard (
    player_id bigint NOT NULL,
    server_id text NOT NULL,
    time_stamp timestamp with time zone NOT NULL,
    id text NOT NULL,
    time_played bigint NOT NULL,
    wins bigint NOT NULL,
    losses bigint NOT NULL,
    kills bigint NOT NULL,
    deaths bigint NOT NULL,
    score bigint NOT NULL,
    name text NOT NULL,
    PRIMARY KEY (id)
);
CREATE INDEX server_idx ON scoreboard (server_id);
CREATE INDEX player_idx ON scoreboard (player_id);
CREATE UNIQUE INDEX idx ON scoreboard (id);


-- CREATE INDEX ON battlefield_servers (time DESC, platform);
-- CREATE INDEX ON battlefield_servers (time DESC, region);
-- CREATE INDEX ON battlefield_servers (time DESC, game);
-- CREATE INDEX ON battlefield_servers (time DESC, servername);
-- CREATE INDEX ON battlefield_servers (time DESC, is_official) WHERE is_official IS NOT NULL;
-- CREATE INDEX ON battlefield_servers (time DESC, game_id) WHERE game_id IS NOT NULL;
-- CREATE INDEX ON battlefield_servers (time DESC, guid) WHERE guid IS NOT NULL;
-- CREATE INDEX ON battlefield_servers (time DESC, game_mode) WHERE game_mode IS NOT NULL;
-- CREATE INDEX ON battlefield_servers (time DESC, game_map) WHERE game_map IS NOT NULL;
-- SELECT * FROM create_hypertable('battlefield_servers', by_range('time', INTERVAL '1 day'));
-- SELECT add_retention_policy('battlefield_servers', INTERVAL '7 days');