import { useMutation, useQuery } from '@tanstack/react-query';
import { Layout } from './components';
import { ContractType, contracts } from './constants';
import { useContract } from './hooks/useContract';

import { CTA, Card, Flex, H1, Input, P, Span } from '@interlay/ui';
import { utils } from 'ethers';
import { FormEventHandler, useEffect, useState } from 'react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { encodeFunctionData } from 'viem';
import { useAccountAbstraction } from './context/AuthContext';

function App() {
  const { safeSelected, relayTransaction } = useAccountAbstraction();
  const { contract, read } = useContract(ContractType.CTF);
  const [isTransfering, setTransfering] = useState(false);
  const [transferAddress, setTransferAddress] = useState('');

  const flagOwner = useQuery<string, Error, string>(['owner'], {
    enabled: !!contract,
    queryFn: () => read('flagHolder') as Promise<string>,
    refetchInterval: 5000
  });

  const capturaFlagMutation = useMutation({
    mutationFn: () => {
      return relayTransaction({
        to: contracts.CTF.address,
        data: encodeFunctionData({
          abi: [
            {
              inputs: [],
              name: 'captureFlag',
              outputs: [{ type: 'address', name: '', internalType: 'address' }],
              stateMutability: 'nonpayable',
              type: 'function'
            }
          ]
        }),
        value: utils.parseUnits('0', 'ether').toString(),
        operation: 0
      });
    },
    onSuccess: () => flagOwner.refetch()
  });

  const transferFlagMutation = useMutation({
    mutationFn: (address: string) => {
      return relayTransaction({
        to: contracts.CTF.address,
        data: encodeFunctionData({
          abi: [
            {
              inputs: [{ internalType: 'address', name: 'newFlagHolder', type: 'address' }],
              name: 'transferOwnership',
              outputs: [],
              stateMutability: 'nonpayable',
              type: 'function'
            }
          ],
          args: [address as `0x${string}`]
        }),
        value: utils.parseUnits('0', 'ether').toString(),
        operation: 0
      });
    },
    onSuccess: () => {
      flagOwner.refetch();
      setTransferAddress('');
    }
  });

  const handleSubmit: FormEventHandler = (e) => {
    e.preventDefault();

    transferFlagMutation.mutate(transferAddress);
  };

  useEffect(() => {
    setTransferAddress('');
    setTransfering(false);

    flagOwner.refetch();
  }, [safeSelected]);

  const isCurrentOwner = safeSelected === flagOwner.data;

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
              <Jazzicon
                diameter={40}
                seed={jsNumberForAddress(flagOwner.data || '0x0000000000000000000000000000000000000000')}
              />
              <Span weight='bold' style={{ color: 'inherit' }} size='xl' color='tertiary'>
                {isCurrentOwner
                  ? 'YOU'
                  : truncateEthAddress(flagOwner.data || '0x0000000000000000000000000000000000000000')}
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
