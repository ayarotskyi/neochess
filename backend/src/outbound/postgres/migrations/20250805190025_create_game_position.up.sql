CREATE TABLE game_position (
    game_id UUID REFERENCES game(id) ON DELETE CASCADE,
    move_idx SMALLINT,
    fen TEXT NOT NULL,
    next_move_uci TEXT
);
CREATE INDEX game_index ON game_position(game_id);
CREATE INDEX fen_index ON game_position(fen);