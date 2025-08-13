DROP INDEX game_index;
DROP INDEX fen_index;
CREATE INDEX game_position_fen_nextmove_idx ON game_position (fen, game_id, next_move_uci)
WHERE next_move_uci IS NOT NULL;
CREATE INDEX game_lowerwhite_platform_finishedat_idx ON game (LOWER(white), platform_name, finished_at);