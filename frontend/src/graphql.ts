import { gql } from '@/__generated__';

const GET_MOVE_STATS = gql(`
  query GetMoveStats(
    $positionFen: String!,
    $username: String!,
    $playAs: Color!,
    $platformName: PlatformName!,
    $fromTimestampSeconds: Int,
    $toTimestampSeconds: Int
  ) {
    getMoveStats(
      positionFen: $positionFen,
      username: $username,
      playAs: $playAs,
      platformName: $platformName,
      fromTimestampSeconds: $fromTimestampSeconds,
      toTimestampSeconds: $toTimestampSeconds,
    ) {
      moveSan
      avgOpponentElo
      wins
      total
      draws
    }
  }
`);
