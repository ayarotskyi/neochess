import { ApolloClient, InMemoryCache } from '@apollo/client';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';
import { split, HttpLink } from '@apollo/client';
import { getMainDefinition } from '@apollo/client/utilities';

const PUBLIC_API_URL: string =
  import.meta.env.PUBLIC_API_URL || 'http://localhost:8000';

const httpLink = new HttpLink({
  uri: `${PUBLIC_API_URL}/graphql`,
});
const wsLink = new GraphQLWsLink(
  createClient({
    url: `${PUBLIC_API_URL.replace(/(^.*):\/\//, 'ws://')}/subscriptions`,
  }),
);

const splitLink = split(
  ({ query }) => {
    const definition = getMainDefinition(query);
    return (
      definition.kind === 'OperationDefinition' &&
      definition.operation === 'subscription'
    );
  },
  wsLink,
  httpLink,
);

const client = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache(),
});

export default client;
