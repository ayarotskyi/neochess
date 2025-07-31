import { ChakraProvider, Stack } from '@chakra-ui/react';
import Root from './components/Root';
import system from './system';
import { ApolloProvider } from '@apollo/client';
import client from './apolloClient';

const App = () => {
  return (
    <ChakraProvider value={system}>
      <ApolloProvider client={client}>
        <Stack height="100vh" flex={1}>
          <Root maxH="100%" />
        </Stack>
      </ApolloProvider>
    </ChakraProvider>
  );
};

export default App;
