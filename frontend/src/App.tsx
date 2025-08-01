import { ChakraProvider, Stack } from '@chakra-ui/react';
import Main from './pages/Main';
import system from './system';
import { ApolloProvider } from '@apollo/client';
import client from './apolloClient';
import { createBrowserRouter, Navigate, RouterProvider } from 'react-router';
import Identification from './pages/Identification';

const router = createBrowserRouter([
  {
    index: true,
    Component: Identification,
  },
  {
    path: '/:platform',
    element: <Navigate to="/" replace />,
  },
  {
    path: '/:platform/:username',
    Component: Main,
  },
]);

const App = () => {
  return (
    <ChakraProvider value={system}>
      <ApolloProvider client={client}>
        <Stack
          height="100vh"
          background="linear-gradient(135deg, rgba(88, 28, 135, 0.5) 15%, #000000 50%, rgba(22, 78, 99, 0.5) 85%)"
          flex={1}
        >
          <RouterProvider router={router} />
        </Stack>
      </ApolloProvider>
    </ChakraProvider>
  );
};

export default App;
