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

const PositionAnalyzer = (props: StackProps) => {
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
          <Button
            bg="rgba(255, 255, 255, 0.002)"
            border="1px solid #C084FC"
            boxShadow="0px 0px 10px rgba(147, 51, 234, 0.5)"
            borderRadius="6px"
            _hover={{
              bg: 'rgba(147, 51, 234, 0.2)',
            }}
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
