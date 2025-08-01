import { Field, Input, type InputProps } from '@chakra-ui/react';

type Props = {
  label?: string;
} & InputProps;

const TextInput = ({ label, ...props }: Props) => {
  return (
    <Field.Root spaceY="0.5rem">
      {label && (
        <Field.Label
          textStyle="mono"
          color="#22d3ee"
          fontSize="0.875rem"
          lineHeight="1.25rem"
          textTransform="uppercase"
        >
          {label}
        </Field.Label>
      )}
      <Input
        py="0.75rem"
        px="1rem"
        borderRadius={0}
        textStyle="mono"
        fontSize="100%"
        border="1px #22d3ee solid"
        bg="rgb(31 41 55 / 0.5)"
        h="fit-content"
        _focus={{
          shadow: '0 0 10px rgba(0,255,255,0.3)',
        }}
        caretColor="#22d3ee"
        color="#67e8f9"
        {...props}
      />
    </Field.Root>
  );
};

export default TextInput;
