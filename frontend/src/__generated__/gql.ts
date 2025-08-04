/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "\n  mutation UpdateUserGames($username: String!, $platformName: PlatformName!) {\n    updateUserGames(username: $username, platformName: $platformName)\n  }\n": typeof types.UpdateUserGamesDocument,
    "\n  query GetMoveStats(\n    $positionFen: String!,\n    $username: String!,\n    $playAs: Color!,\n    $platformName: PlatformName!,\n    $fromTimestampSeconds: Int,\n    $toTimestampSeconds: Int\n  ) {\n    getMoveStats(\n      positionFen: $positionFen,\n      username: $username,\n      playAs: $playAs,\n      platformName: $platformName,\n      fromTimestampSeconds: $fromTimestampSeconds,\n      toTimestampSeconds: $toTimestampSeconds,\n    ) {\n      moveUci\n      avgOpponentElo\n      wins\n      total\n      draws\n      lastPlayedAt\n    }\n  }\n": typeof types.GetMoveStatsDocument,
};
const documents: Documents = {
    "\n  mutation UpdateUserGames($username: String!, $platformName: PlatformName!) {\n    updateUserGames(username: $username, platformName: $platformName)\n  }\n": types.UpdateUserGamesDocument,
    "\n  query GetMoveStats(\n    $positionFen: String!,\n    $username: String!,\n    $playAs: Color!,\n    $platformName: PlatformName!,\n    $fromTimestampSeconds: Int,\n    $toTimestampSeconds: Int\n  ) {\n    getMoveStats(\n      positionFen: $positionFen,\n      username: $username,\n      playAs: $playAs,\n      platformName: $platformName,\n      fromTimestampSeconds: $fromTimestampSeconds,\n      toTimestampSeconds: $toTimestampSeconds,\n    ) {\n      moveUci\n      avgOpponentElo\n      wins\n      total\n      draws\n      lastPlayedAt\n    }\n  }\n": types.GetMoveStatsDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation UpdateUserGames($username: String!, $platformName: PlatformName!) {\n    updateUserGames(username: $username, platformName: $platformName)\n  }\n"): (typeof documents)["\n  mutation UpdateUserGames($username: String!, $platformName: PlatformName!) {\n    updateUserGames(username: $username, platformName: $platformName)\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query GetMoveStats(\n    $positionFen: String!,\n    $username: String!,\n    $playAs: Color!,\n    $platformName: PlatformName!,\n    $fromTimestampSeconds: Int,\n    $toTimestampSeconds: Int\n  ) {\n    getMoveStats(\n      positionFen: $positionFen,\n      username: $username,\n      playAs: $playAs,\n      platformName: $platformName,\n      fromTimestampSeconds: $fromTimestampSeconds,\n      toTimestampSeconds: $toTimestampSeconds,\n    ) {\n      moveUci\n      avgOpponentElo\n      wins\n      total\n      draws\n      lastPlayedAt\n    }\n  }\n"): (typeof documents)["\n  query GetMoveStats(\n    $positionFen: String!,\n    $username: String!,\n    $playAs: Color!,\n    $platformName: PlatformName!,\n    $fromTimestampSeconds: Int,\n    $toTimestampSeconds: Int\n  ) {\n    getMoveStats(\n      positionFen: $positionFen,\n      username: $username,\n      playAs: $playAs,\n      platformName: $platformName,\n      fromTimestampSeconds: $fromTimestampSeconds,\n      toTimestampSeconds: $toTimestampSeconds,\n    ) {\n      moveUci\n      avgOpponentElo\n      wins\n      total\n      draws\n      lastPlayedAt\n    }\n  }\n"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;