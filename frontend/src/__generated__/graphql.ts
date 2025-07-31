/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export enum Color {
  Black = 'BLACK',
  White = 'WHITE'
}

export type GraphQlMoveStat = {
  __typename?: 'GraphQLMoveStat';
  avgOpponentElo: Scalars['Int']['output'];
  draws: Scalars['Int']['output'];
  moveSan: Scalars['String']['output'];
  total: Scalars['Int']['output'];
  wins: Scalars['Int']['output'];
};

/** The root mutation object of the schema */
export type Mutation = {
  __typename?: 'Mutation';
  updateUserGames: Scalars['Int']['output'];
};


/** The root mutation object of the schema */
export type MutationUpdateUserGamesArgs = {
  platformName: PlatformName;
  username: Scalars['String']['input'];
};

export enum PlatformName {
  ChessCom = 'CHESS_COM'
}

/** The root query object of the schema */
export type Query = {
  __typename?: 'Query';
  getMoveStats: Array<GraphQlMoveStat>;
};


/** The root query object of the schema */
export type QueryGetMoveStatsArgs = {
  fromTimestampSeconds?: InputMaybe<Scalars['Int']['input']>;
  platformName: PlatformName;
  playAs: Color;
  positionFen: Scalars['String']['input'];
  toTimestampSeconds?: InputMaybe<Scalars['Int']['input']>;
  username: Scalars['String']['input'];
};

export type GetMoveStatsQueryVariables = Exact<{
  positionFen: Scalars['String']['input'];
  username: Scalars['String']['input'];
  playAs: Color;
  platformName: PlatformName;
  fromTimestampSeconds?: InputMaybe<Scalars['Int']['input']>;
  toTimestampSeconds?: InputMaybe<Scalars['Int']['input']>;
}>;


export type GetMoveStatsQuery = { __typename?: 'Query', getMoveStats: Array<{ __typename?: 'GraphQLMoveStat', moveSan: string, avgOpponentElo: number, wins: number, total: number, draws: number }> };


export const GetMoveStatsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetMoveStats"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"positionFen"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"username"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"playAs"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Color"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"platformName"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"PlatformName"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"fromTimestampSeconds"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"toTimestampSeconds"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"getMoveStats"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"positionFen"},"value":{"kind":"Variable","name":{"kind":"Name","value":"positionFen"}}},{"kind":"Argument","name":{"kind":"Name","value":"username"},"value":{"kind":"Variable","name":{"kind":"Name","value":"username"}}},{"kind":"Argument","name":{"kind":"Name","value":"playAs"},"value":{"kind":"Variable","name":{"kind":"Name","value":"playAs"}}},{"kind":"Argument","name":{"kind":"Name","value":"platformName"},"value":{"kind":"Variable","name":{"kind":"Name","value":"platformName"}}},{"kind":"Argument","name":{"kind":"Name","value":"fromTimestampSeconds"},"value":{"kind":"Variable","name":{"kind":"Name","value":"fromTimestampSeconds"}}},{"kind":"Argument","name":{"kind":"Name","value":"toTimestampSeconds"},"value":{"kind":"Variable","name":{"kind":"Name","value":"toTimestampSeconds"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"moveSan"}},{"kind":"Field","name":{"kind":"Name","value":"avgOpponentElo"}},{"kind":"Field","name":{"kind":"Name","value":"wins"}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"draws"}}]}}]}}]} as unknown as DocumentNode<GetMoveStatsQuery, GetMoveStatsQueryVariables>;