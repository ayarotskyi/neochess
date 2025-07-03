import { HStack, Text, VStack, type StackProps } from '@chakra-ui/react';

const PositionInfo = (props: StackProps) => {
  return (
    <VStack
      bg="rgba(17, 24, 39, 0.5)"
      border="1px solid rgba(34, 197, 94, 0.5)"
      boxShadow="0px 0px 30px rgba(34, 197, 94, 0.3)"
      borderRadius={8}
      padding={25}
      spaceY="24px"
      align="stretch"
      {...props}
    >
      <Text textStyle="sectionHeading" color="#4ADE80">
        Position Info
      </Text>
      <VStack spaceY="12px" align="stretch">
        <HStack>
          <Text
            flex={1}
            textStyle="mono"
            fontWeight={400}
            fontSize="14px"
            lineHeight="20px"
            color="#9CA3AF"
            textTransform="uppercase"
          >
            Games played:
          </Text>
          <Text
            textStyle="mono"
            fontWeight={700}
            fontSize="14px"
            lineHeight="20px"
            color="#4ADE80"
          >
            {1247}
          </Text>
        </HStack>
        <HStack>
          <Text
            flex={1}
            textStyle="mono"
            fontWeight={400}
            fontSize="14px"
            lineHeight="20px"
            color="#9CA3AF"
            textTransform="uppercase"
          >
            Win rate:
          </Text>
          <Text
            textStyle="mono"
            fontWeight={700}
            fontSize="14px"
            lineHeight="20px"
            color="#4ADE80"
          >
            {68.3}%
          </Text>
        </HStack>
        <HStack>
          <Text
            flex={1}
            textStyle="mono"
            fontWeight={400}
            fontSize="14px"
            lineHeight="20px"
            color="#9CA3AF"
            textTransform="uppercase"
          >
            Avg rating:
          </Text>
          <Text
            textStyle="mono"
            fontWeight={700}
            fontSize="14px"
            lineHeight="20px"
            color="#22D3EE"
          >
            {1856}
          </Text>
        </HStack>
        <HStack>
          <Text
            flex={1}
            textStyle="mono"
            fontWeight={400}
            fontSize="14px"
            lineHeight="20px"
            color="#9CA3AF"
            textTransform="uppercase"
          >
            Last played:
          </Text>
          <Text
            textStyle="mono"
            fontWeight={700}
            fontSize="14px"
            lineHeight="20px"
            color="#C084FC"
            textTransform="uppercase"
          >
            2 days ago
          </Text>
        </HStack>
      </VStack>
    </VStack>
  );
};

export default PositionInfo;
