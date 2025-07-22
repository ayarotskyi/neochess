// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Uuid,
        white -> Varchar,
        black -> Varchar,
        platform_name -> Varchar,
        pgn -> Varchar,
    }
}

diesel::table! {
    game_position (game_id, position_id, move_nr) {
        game_id -> Uuid,
        position_id -> Uuid,
        #[max_length = 20]
        move_nr -> Varchar,
    }
}

diesel::table! {
    position (id) {
        id -> Uuid,
        fen -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    game,
    game_position,
    position,
);
