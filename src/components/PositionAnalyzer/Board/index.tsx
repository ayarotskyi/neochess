import { Flex, type FlexProps } from '@chakra-ui/react';
import { RANK_NAMES } from 'chessops';
import { useGameStore } from '@/store/game';
import RankComponent from './RankComponent';
import PromotionPopover from './PromotionPopover';

const Squares = () => {
  const playAs = useGameStore((state) => state.playAs);

  return (
    <Flex
      aspectRatio={1}
      h="100%"
      maxW="100%"
      flexDir={playAs === 'white' ? 'column-reverse' : 'column'}
      align="center"
      bg="rgba(255, 255, 255, 0.002)"
      border="2px solid rgba(6, 182, 212, 0.5)"
      boxShadow="0px 0px 40px rgba(0, 255, 255, 0.2)"
      position="relative"
    >
      {new Array(RANK_NAMES.length).fill(null).map((_, rank) => (
        <RankComponent rank={rank} key={rank} />
      ))}
      <PromotionPopover />
    </Flex>
  );
};

const Board = (props: FlexProps) => {
  return (
    <Flex
      {...props}
      flex={1}
      maxH="100%"
      aspectRatio={1}
      justify="center"
      align="center"
    >
      <Squares />
    </Flex>
  );
};

export default Board;
