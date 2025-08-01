import { PlatformName } from '@/__generated__/graphql';
import { Flex, HStack, Text, VStack, type StackProps } from '@chakra-ui/react';
import { memo, useCallback } from 'react';

type Props = {
  platformName: PlatformName;
  isSelected: boolean;
  selectPlatform: (platformName: PlatformName) => void;
} & StackProps;

const PLATFORM_COLORS: Record<PlatformName, string> = {
  [PlatformName.ChessCom]: '#22c55e',
};

export const PLATFORM_DISPLAY_NAMES: Record<PlatformName, string> = {
  [PlatformName.ChessCom]: 'Chess.com',
};

const PlatformButton = ({
  platformName,
  isSelected,
  selectPlatform,
  ...props
}: Props) => {
  const color = PLATFORM_COLORS[platformName];
  const displayName = PLATFORM_DISPLAY_NAMES[platformName];

  const onClick = useCallback(() => {
    selectPlatform(platformName);
  }, [platformName, selectPlatform]);

  return (
    <HStack
      cursor="pointer"
      p="1rem"
      border={`2px solid ${isSelected ? color : 'rgb(75 85 99)'}`}
      boxShadow={isSelected ? `0 0 20px ${color}66` : undefined}
      bg={isSelected ? `${color}33` : 'rgb(31 41 55 / 0.5)'}
      onClick={onClick}
      borderRadius="0.5rem"
      _hover={
        !isSelected
          ? {
              borderColor: color,
              bg: `${color}19`,
            }
          : undefined
      }
      transition="all 300ms ease"
      spaceX="0.75rem"
      userSelect="none"
      {...props}
    >
      <Flex w="2rem" h="2rem" bg={color} align="center" justify="center">
        <Text textStyle="mono" fontWeight={700} fontSize="100%" color="white">
          {displayName[0]}
        </Text>
      </Flex>
      <VStack align="start">
        <Text textStyle="mono" fontWeight={700} fontSize="100%" color="white">
          {displayName}
        </Text>
        <Text
          textStyle="mono"
          fontSize="0.875rem"
          lineHeight="1.25rem"
          color="rgb(156 163 175)"
        >
          Connect your {displayName} account
        </Text>
      </VStack>
    </HStack>
  );
};

export default memo(PlatformButton);
