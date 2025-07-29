CREATE TABLE game_position (
    game_id UUID REFERENCES game(id),
    position_id UUID REFERENCES position(id),
    move_idx SMALLINT,
    next_move_san TEXT,
    PRIMARY KEY(game_id, position_id, move_idx)
);