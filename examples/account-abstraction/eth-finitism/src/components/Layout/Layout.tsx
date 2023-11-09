import { Flex } from '@interlay/ui';
import { HTMLAttributes } from 'react';
import { Header } from './Header';
import { StyledMain } from './Layout.styles';

const Layout = (props: HTMLAttributes<unknown>) => (
  <Flex direction='column'>
    <Header />
    <StyledMain direction='column' {...props} />
  </Flex>
);

export { Layout };
