import {
  Button,
  Flex,
  HStack,
  Stack,
  Text,
  VStack,
  type StackProps,
} from '@chakra-ui/react';
import Board from './Board';
import DatePicker from './DatePicker';
import { useGameStore } from '@/store/game';
import SwapIcon from '@/icons/SwapIcon';
import { Tooltip } from '@/components/ui/Tooltip';

const PositionAnalyzer = (props: StackProps) => {
  const reset = useGameStore((state) => state.resetBoard);
  const changeSide = useGameStore((state) => state.changeSide);
  return (
    <Stack
      border="1px solid rgba(6, 182, 212, 0.5)"
      boxShadow="0px 0px 30px rgba(0, 255, 255, 0.3)"
      background="rgba(17, 24, 39, 0.5)"
      borderRadius={8}
      p="24px"
      spaceY="24px"
      justify="stretch"
      {...props}
    >
      <VStack spaceY="9px" align="stretch">
        <Text textStyle="sectionHeading" color="#22D3EE">
          Position analyzer
        </Text>
        <HStack justify="space-between">
          <HStack spaceX="1rem">
            <Button
              bg="rgba(255, 255, 255, 0.002)"
              border="1px solid #C084FC"
              boxShadow="0px 0px 10px rgba(147, 51, 234, 0.5)"
              borderRadius="6px"
              _hover={{
                bg: 'rgba(147, 51, 234, 0.2)',
              }}
              onClick={reset}
            >
              <Text
                textStyle="default"
                fontWeight={500}
                fontSize="14px"
                lineHeight="20px"
                color="#FFFFFF"
                textTransform="uppercase"
              >
                Reset
              </Text>
            </Button>
            <Tooltip openDelay={200} closeDelay={200} content="Flip board">
              <Button
                p="9px"
                bg="rgba(17, 24, 39, 0.5)"
                border="1px solid rgba(6, 182, 212, 0.5)"
                boxShadow="0px 0px 10px rgba(0, 255, 255, 0.2)"
                _hover={{
                  bg: 'rgba(31, 41, 55, 0.5)',
                }}
                borderRadius="0px"
                onClick={changeSide}
              >
                <SwapIcon color="#22d3ee" />
              </Button>
            </Tooltip>
          </HStack>
          <DatePicker />
        </HStack>
      </VStack>
      <Flex flex={1} justify="center" align="center" minH="0px" minW="500px">
        <Board />
      </Flex>
    </Stack>
  );
};

export default PositionAnalyzer;
