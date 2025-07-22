CREATE TABLE game (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    white VARCHAR NOT NULL,
    black VARCHAR NOT NULL,
    platform_name VARCHAR NOT NULL,
    pgn VARCHAR NOT NULL
);