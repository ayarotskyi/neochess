CREATE TABLE game_position (
    game_id UUID REFERENCES game(id) ON DELETE CASCADE,
    position_id UUID REFERENCES position(id) ON DELETE CASCADE,
    move_idx SMALLINT,
    next_move_uci TEXT,
    PRIMARY KEY(game_id, position_id, move_idx)
);
CREATE INDEX game_index ON game_position(game_id);
CREATE INDEX postiion_index ON game_position(position_id);