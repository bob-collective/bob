import { CTA, Flex, Span } from '@interlay/ui';
import { useWeb3Modal } from '@web3modal/wagmi/react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { useAccount } from 'wagmi';
import { CTAWrapper, StyledHeader } from './Layout.styles';
import { EthFaucet } from '../EthFaucet';

const Header = () => {
  const { open } = useWeb3Modal();
  const { address, isConnecting } = useAccount();

  return (
    <StyledHeader elementType='header' alignItems='center' justifyContent='space-between'>
      <a href='/' aria-label='navigate to home page'>
        <img
          src='https://uploads-ssl.webflow.com/64e85c2f3609488b3ed725f4/64ede4ad095a0a3801df095f_BobLogo.svg'
          width='137'
          alt='logo'
        />
      </a>
      <CTAWrapper>
        {address && (
          <>
            <EthFaucet />
          </>
        )}
        <CTA disabled={isConnecting} size='small' onClick={() => open()}>
          {address ? (
            <Flex elementType='span' gap='spacing2'>
              <Jazzicon diameter={20} seed={jsNumberForAddress(address)} />
              <Span style={{ color: 'inherit' }} size='s' color='tertiary'>
                {truncateEthAddress(address)}
              </Span>
            </Flex>
          ) : (
            'Connect Wallet'
          )}
        </CTA>
      </CTAWrapper>
    </StyledHeader>
  );
};

export { Header };
