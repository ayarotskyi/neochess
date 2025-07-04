import { useGameStore } from '@/store/game';
import { Flex, type FlexProps } from '@chakra-ui/react';
import { FILE_NAMES, parseSquare, RANK_NAMES, type SquareName } from 'chessops';
import { memo } from 'react';
import SquareComponent from './SquareComponent';

type Props = FlexProps & {
  rank: number;
};

const RankComponent = ({ rank, ...props }: Props) => {
  const playAs = useGameStore((state) => state.playAs);
  return (
    <Flex
      h="12.5%"
      direction={playAs === 'white' ? 'row' : 'row-reverse'}
      {...props}
    >
      {FILE_NAMES.map((fileName) => {
        const squareName: SquareName = `${fileName}${RANK_NAMES[rank]}`;
        const square = parseSquare(squareName);
        return <SquareComponent square={square} key={square} />;
      })}
    </Flex>
  );
};

export default memo(RankComponent);
