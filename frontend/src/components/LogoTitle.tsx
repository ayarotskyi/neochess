import { Text, type TextProps } from '@chakra-ui/react';

type Props = TextProps;

const LogoTitle = (props: Props) => {
  return (
    <Text
      fontSize="3rem"
      lineHeight="1"
      fontWeight={700}
      color="transparent"
      backgroundImage="linear-gradient(to right, #A78BFA, #F472B6)"
      backgroundClip="text"
      textTransform="uppercase"
      {...props}
    >
      Neochess
    </Text>
  );
};

export default LogoTitle;
