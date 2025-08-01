import { PlatformName } from '@/__generated__/graphql';
import PlatformButton from '@/components/Identification/PlatformButton';
import { Flex, Stack, Text, VStack } from '@chakra-ui/react';
import { useMemo, useState } from 'react';

const Identification = () => {
  const [selectedPlatform, setSelectedPlatform] = useState<PlatformName>();

  const Platforms = useMemo(
    () =>
      Object.values(PlatformName).map((platformName) => (
        <PlatformButton
          platformName={platformName}
          isSelected={platformName === selectedPlatform}
          key={platformName}
          selectPlatform={setSelectedPlatform}
        />
      )),
    [selectedPlatform],
  );

  return (
    <Flex flex={1} align="center" justify="center">
      <Stack
        p="1.5rem"
        border="1px rgb(6 182 212 / 0.5) solid"
        borderRadius="0.5rem"
        boxShadow="0 0 #0000, 0 0 #0000, 0 0 30px rgba(0,255,255,0.3)"
        bg="rgb(17 24 39 / 0.5)"
        width="450px"
        align="stretch"
      >
        <Text
          textStyle="sectionHeading"
          color="rgb(34 211 238)"
          pb="1.5rem"
          textAlign="center"
        >
          Select platform
        </Text>
        <VStack spaceY="0.75rem" align="stretch">
          {Platforms}
        </VStack>
      </Stack>
    </Flex>
  );
};

export default Identification;
