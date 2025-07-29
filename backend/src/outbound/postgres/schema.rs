// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Uuid,
        white -> Varchar,
        white_elo -> Int2,
        black -> Varchar,
        black_elo -> Int2,
        #[max_length = 5]
        winner -> Nullable<Bpchar>,
        platform_name -> Varchar,
        pgn -> Varchar,
        finished_at -> Timestamptz,
    }
}

diesel::table! {
    game_position (game_id, position_id, move_idx) {
        game_id -> Uuid,
        position_id -> Uuid,
        move_idx -> Int2,
        next_move_san -> Nullable<Text>,
    }
}

diesel::table! {
    position (id) {
        id -> Uuid,
        fen -> Varchar,
    }
}

diesel::joinable!(game_position -> game (game_id));
diesel::joinable!(game_position -> position (position_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    game_position,
    position,
);
