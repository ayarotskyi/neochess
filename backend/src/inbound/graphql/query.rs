use chrono::DateTime;
use juniper::{FieldResult, graphql_object};

use crate::{
    domain::game::models::{
        game::GameRepositoryError,
        position::{InvalidFenError, MoveStat},
    },
    inbound::graphql::{
        GraphQLContext,
        dto::{GraphQLColor, GraphQLMoveStat, GraphQLPlatformName},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn get_move_stats(
        #[graphql(context)] ctx: &GraphQLContext,
        position_fen: String,
        username: String,
        play_as: GraphQLColor,
        platform_name: GraphQLPlatformName,
        from_timestamp_seconds: Option<i32>,
        to_timestamp_seconds: Option<i32>,
    ) -> FieldResult<Vec<GraphQLMoveStat>> {
        let move_stats: Result<Vec<MoveStat>, GetMoveStatsError> = ctx
            .game_service
            .get_move_stats(
                ctx.game_service.parse_fen(position_fen)?,
                username,
                play_as.into(),
                platform_name.into(),
                match from_timestamp_seconds {
                    Some(from_timestamp_seconds) => Some(
                        DateTime::from_timestamp(from_timestamp_seconds as i64, 0).ok_or(
                            GetMoveStatsError::InvalidTimestamp(
                                "from_timestamp_seconds".to_string(),
                            ),
                        )?,
                    ),
                    None => None,
                },
                match to_timestamp_seconds {
                    Some(to_timestamp_seconds) => Some(
                        DateTime::from_timestamp(to_timestamp_seconds as i64, 0).ok_or(
                            GetMoveStatsError::InvalidTimestamp("to_timestamp_seconds".to_string()),
                        )?,
                    ),
                    None => None,
                },
            )
            .await
            .map_err(|e| {
                println!("{}", e.to_string());
                e.into()
            });

        Ok(move_stats?
            .into_iter()
            .map(|move_stat| move_stat.into())
            .collect::<_>())
    }
}

#[derive(Debug, thiserror::Error)]
enum GetMoveStatsError {
    #[error("Internal error")]
    InternalError,
    #[error(transparent)]
    InvalidFen(#[from] InvalidFenError),
    #[error("Invalid timestamp for column {0}")]
    InvalidTimestamp(String),
}

impl From<GameRepositoryError> for GetMoveStatsError {
    fn from(_: GameRepositoryError) -> Self {
        Self::InternalError
    }
}
