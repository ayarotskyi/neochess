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
  mono: {
    value: {
      fontFamily: 'Geist Mono Variable',
    },
  },
  default: {
    value: {
      fontFamily: 'Geist Variable',
    },
  },
});

const config = defineConfig({
  theme: {
    textStyles,
  },
  globalCss: {
    html: {
      backgroundColor: 'black',
      overflow: 'hidden',
    },
  },
});

const system = createSystem(defaultConfig, config);

export default system;
