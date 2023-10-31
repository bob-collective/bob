import { theme } from '@interlay/theme';
import { Flex } from '@interlay/ui';
import styled from 'styled-components';

const StyledHeader = styled(Flex)`
  padding: 0 ${theme.spacing.spacing4};

  @media ${theme.breakpoints.up('md')} {
    padding: 0 ${theme.spacing.spacing12};
  }
`;

const StyledMain = styled(Flex)`
  margin: ${theme.spacing.spacing6} auto;
  width: 100%;
  padding: 0 ${theme.spacing.spacing4};
  max-width: ${theme.breakpoints.values.lg}px;

  @media ${theme.breakpoints.up('md')} {
    padding: 0 ${theme.spacing.spacing12};
  }
`;

const CTAWrapper = styled(Flex)`
  gap: ${theme.spacing.spacing2};
`;

export { StyledHeader, StyledMain, CTAWrapper };
