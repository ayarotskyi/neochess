import {
  Spinner,
  Stack,
  Text,
  VStack,
  type StackProps,
} from '@chakra-ui/react';
import LogoTitle from '../LogoTitle';
import { useMutation } from '@apollo/client';
import { gql } from '@/__generated__';
import { useCallback, useContext, useEffect } from 'react';
import { useNavigate } from 'react-router';
import { PLATFORM_DISPLAY_NAMES } from '@/constants';
import { toaster } from '../ui/toaster';
import ParamsContext from '@/contexts/ParamsContext';

type Props = StackProps & {
  onDataLoaded: () => void;
};

const UPDATE_USER_GAMES = gql(`
  mutation UpdateUserGames($username: String!, $platformName: PlatformName!) {
    updateUserGames(username: $username, platformName: $platformName)
  }
`);

const GameLoader = ({ onDataLoaded, ...props }: Props) => {
  const { username, platformName } = useContext(ParamsContext);

  const [mutate] = useMutation(UPDATE_USER_GAMES, {
    variables: {
      username,
      platformName,
    },
  });

  const navigate = useNavigate();

  const onError = useCallback(() => {
    navigate('/');
    toaster.create({
      title: `Failed to fetch games for @${username}`,
      type: 'error',
    });
  }, [navigate, username]);

  useEffect(() => {
    (async () => {
      try {
        const result = await mutate();
        if (result.data?.updateUserGames !== undefined) {
          onDataLoaded();
        } else {
          onError();
        }
      } catch {
        onError();
      }
    })();
  }, [mutate, onDataLoaded, onError]);

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
      </Stack>
    </VStack>
  );
};

export default GameLoader;
