CREATE TABLE game (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    white VARCHAR NOT NULL,
    white_elo SMALLINT NOT NULL,
    black VARCHAR NOT NULL,
    black_elo SMALLINT NOT NULL,
    platform_name VARCHAR NOT NULL,
    pgn VARCHAR NOT NULL,
    finished_at TIMESTAMP WITH TIME ZONE NOT NULL
);