import { PlatformName } from '@/__generated__/graphql';
import { createContext } from 'react';

const ParamsContext = createContext<{
  username: string;
  platformName: PlatformName;
}>({
  username: '',
  platformName: PlatformName.ChessCom,
});

export default ParamsContext;
