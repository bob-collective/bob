import { createWeb3Modal } from '@web3modal/wagmi/react';
import { useAccount, useMutation, useQuery } from 'wagmi';
import { L2_CHAIN_CONFIG, L2_PROJECT_ID, config } from './connectors/wagmi-connectors';

import { CTA, Card, Flex, H1, Input, P, Span } from '@interlay/ui';
import { Layout } from './components';
import { ContractType } from './constants';
import { useContract } from './hooks/useContract';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { FormEventHandler, useEffect, useState } from 'react';

createWeb3Modal({
  defaultChain: L2_CHAIN_CONFIG,
  wagmiConfig: config,
  projectId: L2_PROJECT_ID,
  chains: config.chains
});

function App() {
  const { read, write } = useContract(ContractType.CTF);
  const { address } = useAccount();
  const [isTransfering, setTransfering] = useState(false);
  const [transferAddress, setTransferAddress] = useState('');

  const flagOwner = useQuery<string, Error, string>(['owner'], {
    queryFn: () => read.flagHolder() as Promise<string>,
    refetchInterval: 10000
  });

  const capturaFlagMutation = useMutation({
    mutationFn: () => write.captureFlag(),
    onSuccess: () => flagOwner.refetch()
  });

  const transferFlagMutation = useMutation({
    mutationFn: (address: string) => write.transferOwnership([address]),
    onSuccess: () => {
      flagOwner.refetch();
      setTransferAddress('');
    }
  });

  const handleSubmit: FormEventHandler = (e) => {
    e.preventDefault();

    transferFlagMutation.mutate(transferAddress);
  };

  const isCurrentOwner = address === flagOwner.data;

  useEffect(() => {
    setTransferAddress('');
    setTransfering(false);
  }, [address]);

  return (
    <Layout>
      <Card
        alignItems='center'
        justifyContent='space-between'
        gap='spacing10'
        style={{ margin: 'auto', maxWidth: 500, width: '100%' }}
      >
        <Flex direction='column' gap='spacing4'>
          <H1 align='center' size='xl3'>
            Capture the Flag
          </H1>
          <Flex direction='column' alignItems='center' gap='spacing1'>
            <P>Current Holder</P>
            <Flex elementType='span' gap='spacing2' alignItems='center'>
              <Jazzicon diameter={40} seed={jsNumberForAddress(flagOwner.data || '0x0') || '0'} />
              <Span weight='bold' style={{ color: 'inherit' }} size='xl' color='tertiary'>
                {isCurrentOwner ? 'YOU' : truncateEthAddress(flagOwner.data || '0x0')}
              </Span>
              {isCurrentOwner && (
                <CTA size='x-small' onPress={() => setTransfering(true)}>
                  Transfer
                </CTA>
              )}
            </Flex>
          </Flex>
        </Flex>
        {isTransfering && (
          <Flex elementType='form' style={{ width: '100%' }} gap='spacing1' onSubmit={handleSubmit}>
            <Input
              aria-label='address'
              size='small'
              style={{ width: '100%', flex: 1 }}
              placeholder='Enter next owner address'
              onValueChange={(address) => setTransferAddress(address)}
            />
            <CTA loading={transferFlagMutation.isLoading} type='submit' size='small'>
              Transfer
            </CTA>
            <CTA
              disabled={transferFlagMutation.isLoading}
              onPress={() => {
                setTransfering(false);
                setTransferAddress('');
              }}
              variant='secondary'
              size='small'
            >
              Cancel
            </CTA>
          </Flex>
        )}
        <CTA
          loading={capturaFlagMutation.isLoading}
          disabled={flagOwner.isLoading || isCurrentOwner || transferFlagMutation.isLoading}
          size='large'
          fullWidth
          onPress={() => capturaFlagMutation.mutate()}
        >
          Capture
        </CTA>
      </Card>
    </Layout>
  );
}

export default App;
