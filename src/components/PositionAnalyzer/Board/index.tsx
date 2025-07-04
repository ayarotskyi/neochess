import { Flex, type FlexProps } from '@chakra-ui/react';
import { FILE_NAMES, parseSquare, RANK_NAMES, type SquareName } from 'chessops';
import SquareComponent from './SquareComponent';
import { useGameStore } from '@/store/game';
import { parseFen } from 'chessops/fen';

const Squares = () => {
  const playAs = useGameStore((state) => state.playAs);
  const fen = useGameStore((state) => state.fen);

  const board = parseFen(fen).unwrap().board;

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
    >
      {RANK_NAMES.map((rank) =>
        FILE_NAMES.map((file) => `${file}${rank}` as SquareName),
      ).map((squareNames) => {
        return (
          <Flex
            h="12.5%"
            direction={playAs === 'white' ? 'row' : 'row-reverse'}
            key={squareNames[0]}
          >
            {squareNames.map((squareName) => {
              const square = parseSquare(squareName);
              const piece = board.get(square);
              return (
                <SquareComponent
                  square={square}
                  key={squareName}
                  piece={piece}
                />
              );
            })}
          </Flex>
        );
      })}
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
