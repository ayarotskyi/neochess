import { HStack, type StackProps } from '@chakra-ui/react';
import Statistic from '../components/Main/Statistic';
import PositionAnalyzer from '../components/Main/PositionAnalyzer';
import { Navigate, useParams } from 'react-router';
import { useCallback, useState } from 'react';
import GameLoader from '@/components/Main/GameLoader';
import { PLATFORM_URLS } from '@/constants';
import type { PlatformName } from '@/__generated__/graphql';

const Main = (props: StackProps) => {
  const { username, platformName: platformNameString } = useParams();

  const platformName = Object.keys(PLATFORM_URLS).find(
    (key) => platformNameString === PLATFORM_URLS[key as PlatformName],
  ) as PlatformName | undefined;

  const [isDataLoaded, setDataLoaded] = useState(false);

  const onDataLoaded = useCallback(() => {
    setDataLoaded(true);
  }, []);

  return !username || !platformName ? (
    <Navigate to="/" replace />
  ) : !isDataLoaded ? (
    <GameLoader
      onDataLoaded={onDataLoaded}
      username={username}
      platformName={platformName}
    />
  ) : (
    <HStack
      align="stretch"
      flex={1}
      spaceX="48px"
      px="10%"
      py={5}
      overflow="hidden"
      maxH="100%"
      {...props}
    >
      <PositionAnalyzer flex={5} />
      <Statistic flex={2} />
    </HStack>
  );
};

export default Main;
