CREATE TABLE game_position (
    game_id UUID REFERENCES game(id),
    position_id UUID REFERENCES game(id),
    move_nr VARCHAR(20),
    PRIMARY KEY(game_id, position_id, move_nr)
);