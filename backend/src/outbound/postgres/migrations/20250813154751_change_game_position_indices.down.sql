DROP INDEX game_position_fen_nextmove_idx;
DROP INDEX game_lowerwhite_platform_finishedat_idx;
CREATE INDEX game_index ON game_position(game_id);
CREATE INDEX fen_index ON game_position(fen);