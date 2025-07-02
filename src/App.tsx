import { ChakraProvider, defaultSystem, Stack } from '@chakra-ui/react';
import Root from './components/Root';

const App = () => {
  return (
    <ChakraProvider value={defaultSystem}>
      <Stack height="100vh" flex={1} bg="black">
        <Stack
          position="absolute"
          height="100%"
          width="100%"
          bg="linear-gradient(135deg, rgba(88, 28, 135, 0.2) 0%, #000000 50%, rgba(22, 78, 99, 0.2) 100%)"
          zIndex={1}
        />
        <Root zIndex={2} />
      </Stack>
    </ChakraProvider>
  );
};

export default App;
