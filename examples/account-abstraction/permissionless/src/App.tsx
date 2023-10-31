import { createWeb3Modal } from '@web3modal/wagmi/react';
import { useAccount, useMutation, useQuery, useSignMessage } from 'wagmi';
import { L2_CHAIN_CONFIG, L2_PROJECT_ID, config } from './connectors/wagmi-connectors';

import { CTA, Card, Flex, H1, Input, P, Span } from '@interlay/ui';
import { UserOperation, getSenderAddress, getUserOperationHash } from 'permissionless';
import { FormEventHandler, useEffect, useState } from 'react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { Hex, encodeFunctionData } from 'viem';
import { goerli } from 'viem/chains';
import { Layout } from './components';
import { ContractType } from './constants';
import { ENTRY_POINT_ADDRESS } from './constants/erc4337';
import { useContract } from './hooks/useContract';
import { bundlerClient, getInitCode, publicClient } from './sdk';

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
  const { signMessageAsync } = useSignMessage();

  const flagOwner = useQuery<string, Error, string>(['owner'], {
    queryFn: () => read.flagHolder() as Promise<string>,
    refetchInterval: 5000
  });

  const capturaFlagMutation = useMutation({
    // mutationFn: () => write.captureFlag(),
    mutationFn: async () => {
      if (!address) return;

      const initCode = getInitCode(address);

      const senderAddress = await getSenderAddress(publicClient, {
        initCode,
        entryPoint: ENTRY_POINT_ADDRESS
      });

      const callData = encodeFunctionData({
        abi: [
          {
            inputs: [],
            name: 'captureFlag',
            outputs: [{ type: 'address', name: '', internalType: 'address' }],
            stateMutability: 'nonpayable',
            type: 'function'
          }
        ]
      });

      const gasPrice = await bundlerClient.getUserOperationGasPrice();

      const userOperation = {
        sender: senderAddress,
        nonce: 2n,
        // NOTE: should pass this if the account was not created yet
        // initCode: initCode
        // NOT  putting "0x" because account was already created
        initCode: '0x',
        callData,
        maxFeePerGas: gasPrice.fast.maxFeePerGas,
        maxPriorityFeePerGas: gasPrice.fast.maxPriorityFeePerGas,
        // dummy signature
        signature:
          '0xa15569dd8f8324dbeabf8073fdec36d4b754f53ce5901e283c6de79af177dc94557fa3c9922cd7af2a96ca94402d35c39f266925ee6407aeb32b31d76978d4ba1c' as Hex
      };

      // NOTE: code for sponsoring the userOP
      // const sponsorUserOperationResult = await paymasterClient.sponsorUserOperation({
      //   userOperation,
      //   entryPoint: ENTRY_POINT_ADDRESS
      // });

      // const sponsoredUserOp: UserOperation = {
      //   ...userOperation,
      //   ...sponsorUserOperationResult
      // };

      const finalUserOperation: UserOperation = {
        ...userOperation,
        callGasLimit: 60000n,
        verificationGasLimit: 60000n,
        preVerificationGas: 60000n,
        paymasterAndData: '0x'
      };

      const signature = await signMessageAsync({
        message: {
          /** Raw data representation of the message. */
          raw: getUserOperationHash({
            userOperation: finalUserOperation,
            chainId: goerli.id,
            entryPoint: ENTRY_POINT_ADDRESS
          })
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
        } as unknown as any
      });

      finalUserOperation.signature = signature;

      // SUBMIT THE USER OPERATION TO BE BUNDLED
      const userOperationHash = await bundlerClient.sendUserOperation({
        userOperation: finalUserOperation,
        entryPoint: ENTRY_POINT_ADDRESS
      });

      console.log('Received User Operation hash:', userOperationHash);

      // let's also wait for the userOperation to be included, by continually querying for the receipts
      console.log('Querying for receipts...');
      const receipt = await bundlerClient.waitForUserOperationReceipt({ hash: userOperationHash });
      const txHash = receipt.receipt.transactionHash;

      console.log(`UserOperation included: https://goerli.lineascan.build/tx/${txHash}`);
    },
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
              <Jazzicon diameter={40} seed={jsNumberForAddress(flagOwner.data || '0x0')} />
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
