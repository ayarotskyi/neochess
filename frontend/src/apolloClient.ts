import { ApolloClient, InMemoryCache } from '@apollo/client';

const client = new ApolloClient({
  uri: `${import.meta.env.PUBLIC_API_URL || 'http://localhost:8000'}/graphql`,
  cache: new InMemoryCache(),
});

export default client;
