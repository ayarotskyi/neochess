import {
  Flex,
  Progress,
  Spinner,
  Stack,
  Text,
  VStack,
  type StackProps,
} from '@chakra-ui/react';
import LogoTitle from '../LogoTitle';
import { useSubscription } from '@apollo/client';
import { gql } from '@/__generated__';
import { useContext } from 'react';
import { useNavigate } from 'react-router';
import { PLATFORM_DISPLAY_NAMES } from '@/constants';
import { toaster } from '../ui/toaster';
import ParamsContext from '@/contexts/ParamsContext';

type Props = StackProps & {
  onComplete: () => void;
};

const UPDATE_USER_GAMES = gql(`
  subscription UpdateUserGames($username: String!, $platformName: PlatformName!) {
    updateUserGames(username: $username, platformName: $platformName)
  }
`);

const GameLoader = ({ onComplete, ...props }: Props) => {
  const { username, platformName } = useContext(ParamsContext);

  const navigate = useNavigate();

  const { data } = useSubscription(UPDATE_USER_GAMES, {
    variables: {
      username,
      platformName,
    },
    onError: () => {
      navigate('/');
      toaster.create({
        title: `Failed to fetch games for @${username}`,
        type: 'error',
      });
    },
    onComplete: () => {
      onComplete();
    },
  });

  return (
    <VStack flex={1} spaceY="2rem" align="center" justify="center" {...props}>
      <LogoTitle />
      <Stack
        p="1.5rem"
        border="1px rgb(6 182 212 / 0.5) solid"
        borderRadius="0.5rem"
        boxShadow="0 0 #0000, 0 0 #0000, 0 0 30px rgba(0,255,255,0.3)"
        bg="rgb(17 24 39 / 0.5)"
        width="400px"
        align="center"
        spaceY="1rem"
      >
        <Spinner size="lg" color="#22d3ee" />
        <Text color="#22d3ee" textStyle="mono">
          Connecting to {PLATFORM_DISPLAY_NAMES[platformName]}...
        </Text>
        <Text
          color="#9ca3af"
          textStyle="mono"
          fontSize="0.875rem"
          lineHeight="1.25rem"
          textAlign="center"
        >
          Fetching games for @{username}
        </Text>
        <Flex alignSelf="stretch">
          <Progress.Root
            defaultValue={0}
            value={(data?.updateUserGames ?? 0) * 100}
            flex={1}
          >
            <Progress.Track>
              <Progress.Range bg="linear-gradient(to right, #06b6d4, #a855f7)" />
            </Progress.Track>
          </Progress.Root>
        </Flex>
      </Stack>
    </VStack>
  );
};

export default GameLoader;
