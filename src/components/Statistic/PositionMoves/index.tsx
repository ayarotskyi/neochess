import { Box, Flex, Text, VStack, type StackProps } from '@chakra-ui/react';
import MoveStatistics from './MoveStatistics';
import { useAnalyzerStore } from '@/store/analyzer';

const PositionMoves = (props: StackProps) => {
  const moveStatisticsLength = useAnalyzerStore(
    (state) => state.moveStatistics.length,
  );
  return (
    <Flex
      bg="rgba(17, 24, 39, 0.5)"
      border="1px solid rgba(168, 85, 247, 0.5)"
      boxShadow="0px 0px 30px rgba(147, 51, 234, 0.3)"
      borderRadius={8}
      padding="24px"
      align="stretch"
      spaceY="24px"
      flexDir="column"
      overflow="auto"
      flex={1}
      className="scroller"
      {...props}
    >
      <Text textStyle="sectionHeading" color="#C084FC">
        Move Statistics
      </Text>
      <VStack spaceY="16px" align="stretch" flex={1}>
        {new Array(moveStatisticsLength).fill(null).map((_, statIndex) => {
          return <MoveStatistics statIndex={statIndex} key={statIndex} />;
        })}
      </VStack>
      <Box flex={1}></Box>
    </Flex>
  );
};

export default PositionMoves;
