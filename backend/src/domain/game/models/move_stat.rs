use chrono::{DateTime, Utc};

pub struct MoveStat {
    move_uci: String,
    total: u64,
    wins: u64,
    draws: u64,
    avg_opponent_elo: u16,
    last_played_at: DateTime<Utc>,
}

impl MoveStat {
    pub fn new(
        move_uci: String,
        total: u64,
        wins: u64,
        draws: u64,
        avg_opponent_elo: u16,
        last_played_at: DateTime<Utc>,
    ) -> Self {
        Self {
            move_uci,
            total,
            wins,
            draws,
            avg_opponent_elo,
            last_played_at,
        }
    }

    pub fn move_uci(&self) -> &str {
        &self.move_uci
    }

    pub fn total(&self) -> &u64 {
        &self.total
    }

    pub fn wins(&self) -> &u64 {
        &self.wins
    }

    pub fn draws(&self) -> &u64 {
        &self.draws
    }

    pub fn avg_opponent_elo(&self) -> &u16 {
        &self.avg_opponent_elo
    }

    pub fn last_played_at(&self) -> &DateTime<Utc> {
        &self.last_played_at
    }
}
