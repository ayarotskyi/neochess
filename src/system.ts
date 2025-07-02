import {
  createSystem,
  defaultConfig,
  defineConfig,
  defineTextStyles,
} from '@chakra-ui/react';

const textStyles = defineTextStyles({
  sectionHeading: {
    value: {
      fontFamily: 'Geist Mono Variable',
      fontWeight: '600',
      fontSize: '20px',
      lineHeight: '28px',
      letterSpacing: '-0.5px',
      textTransform: 'uppercase',
    },
  },
});

const config = defineConfig({
  theme: {
    textStyles,
  },
});

const system = createSystem(defaultConfig, config);

export default system;
