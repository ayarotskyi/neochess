import { PlatformName } from '@/__generated__/graphql';
import PlatformButton from '@/components/Identification/PlatformButton';
import LogoTitle from '@/components/LogoTitle';
import { PLATFORM_DISPLAY_NAMES, PLATFORM_URLS } from '@/constants';
import TextInput from '@/ui/TextInput';
import { Button, Stack, Text, VStack } from '@chakra-ui/react';
import { useCallback, useMemo, useState } from 'react';
import { useNavigate } from 'react-router';

const Identification = () => {
  const [selectedPlatform, setSelectedPlatform] = useState<PlatformName>();
  const [username, setUsername] = useState<string>();

  const navigate = useNavigate();

  const submit = useCallback(() => {
    if (!selectedPlatform || !username) {
      return;
    }

    navigate(`/${PLATFORM_URLS[selectedPlatform]}/${username}`);
  }, [navigate, selectedPlatform, username]);

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
    <VStack flex={1} spaceY="2rem" align="center" justify="center">
      <LogoTitle />
      <Stack
        p="1.5rem"
        border="1px rgb(6 182 212 / 0.5) solid"
        borderRadius="0.5rem"
        boxShadow="0 0 #0000, 0 0 #0000, 0 0 30px rgba(0,255,255,0.3)"
        bg="rgb(17 24 39 / 0.5)"
        width="450px"
        align="stretch"
        spaceY="1.5rem"
      >
        <Text
          textStyle="sectionHeading"
          color="rgb(34 211 238)"
          textAlign="center"
        >
          Select platform
        </Text>
        <VStack spaceY="0.75rem" align="stretch">
          {Platforms}
        </VStack>
        {!!selectedPlatform && (
          <TextInput
            label="username"
            placeholder={`Enter your ${PLATFORM_DISPLAY_NAMES[selectedPlatform]} username`}
            onChange={(event) => setUsername(event.target.value)}
          />
        )}
        <Button
          py="0.75rem"
          borderRadius="0.5rem"
          borderWidth="1px"
          textStyle="mono"
          fontWeight={700}
          fontSize="100%"
          color="white"
          backgroundImage="linear-gradient(to right, #06b6d4 , #a855f7)"
          boxShadow="0 0 20px rgba(0,255,255,0.4)"
          _hover={{
            backgroundImage: 'linear-gradient(to right, #00b8db , #ad46ff)',
            boxShadow: '0 0 20px rgba(0,255,255,0.6)',
          }}
          border="none"
          textTransform="uppercase"
          lineHeight="1.5rem"
          h="fit-content"
          disabled={!selectedPlatform || !username}
          onClick={submit}
        >
          Connect account
        </Button>
      </Stack>
    </VStack>
  );
};

export default Identification;
