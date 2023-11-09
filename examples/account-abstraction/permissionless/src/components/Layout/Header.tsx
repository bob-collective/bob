import { CTA, Flex, Span } from '@interlay/ui';
import { useWeb3Modal } from '@web3modal/wagmi/react';
import { getSenderAddress } from 'permissionless';
import { useState } from 'react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { useAccount, useBalance } from 'wagmi';
import { ENTRY_POINT_ADDRESS } from '../../constants/erc4337';
import { getInitCode, publicClient } from '../../sdk';
import { CTAWrapper, StyledHeader } from './Layout.styles';

const Header = () => {
  const { open } = useWeb3Modal();

  const [smartContractAddress, setSmartContractAddress] = useState('');

  const { address, isConnecting } = useAccount({
    onConnect: async ({ address }) => {
      if (!address) return;

      console.log(getInitCode(address));

      const senderAddress = await getSenderAddress(publicClient, {
        initCode: getInitCode(address),
        entryPoint: ENTRY_POINT_ADDRESS
      });
      setSmartContractAddress(senderAddress);
    }
  });
  const { data } = useBalance({ address });

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
        <Span>{smartContractAddress}</Span>
        <Span>Balance: {data?.value.toString()}</Span>
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
