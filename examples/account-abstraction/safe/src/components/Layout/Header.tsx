import { CTA, Card, Dd, Dl, DlGroup, Dt, Flex, Modal, ModalBody, ModalFooter, ModalHeader, Span } from '@interlay/ui';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { useAccountAbstraction } from '../../context/AuthContext';
// import { EthFaucet } from '../EthFaucet';
import { CTAWrapper, StyledHeader } from './Layout.styles';
import { useState } from 'react';

const Header = () => {
  const { isAuthenticated, safeSelected, safeBalance, loginWeb3Auth, logoutWeb3Auth } = useAccountAbstraction();
  const [isOpen, setOpen] = useState(false);

  return (
    <StyledHeader elementType='header' alignItems='center' justifyContent='space-between'>
      <a href='/' aria-label='navigate to home page'>
        <img
          src='https://uploads-ssl.webflow.com/64e85c2f3609488b3ed725f4/64ede4ad095a0a3801df095f_BobLogo.svg'
          width='137'
          alt='logo'
        />
      </a>
      <CTAWrapper alignItems='center'>
        {isAuthenticated && safeSelected ? (
          <CTA size='small' onClick={() => setOpen(true)}>
            <Flex elementType='span' gap='spacing2'>
              <Jazzicon diameter={20} seed={jsNumberForAddress(safeSelected)} />
              <Span style={{ color: 'inherit' }} size='s' color='tertiary'>
                {truncateEthAddress(safeSelected)}
              </Span>
            </Flex>
          </CTA>
        ) : (
          <CTA size='small' onClick={() => loginWeb3Auth()}>
            Connect Wallet
          </CTA>
        )}
      </CTAWrapper>
      <Modal isOpen={isOpen} onClose={() => setOpen(false)}>
        <ModalHeader>Details</ModalHeader>
        <ModalBody>
          <Card background='secondary'>
            <Dl direction='column'>
              <DlGroup>
                <Dt size='s'>Address:</Dt>
                <Dd size='xs'>{safeSelected}</Dd>
              </DlGroup>
              <DlGroup>
                <Dt size='s'>Balance:</Dt>
                <Dd size='s'>{Number(safeBalance || 0) / 10 ** 18} ETH</Dd>
              </DlGroup>
            </Dl>
          </Card>
        </ModalBody>
        <ModalFooter>
          <CTA
            fullWidth
            size='large'
            onPress={() => {
              logoutWeb3Auth();
              setOpen(false);
            }}
            variant='secondary'
          >
            Logout
          </CTA>
        </ModalFooter>
      </Modal>
    </StyledHeader>
  );
};

export { Header };
