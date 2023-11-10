import { createWeb3Modal } from '@web3modal/wagmi/react';
import { useAccount, useMutation, usePublicClient, useQuery, useSignMessage } from 'wagmi';
import { L2_CHAIN_CONFIG, L2_PROJECT_ID, config } from './connectors/wagmi-connectors';

import { CTA, Card, Flex, H1, Input, P, Span } from '@interlay/ui';
import { UserOperation, getAccountNonce, getSenderAddress, getUserOperationHash } from 'permissionless';
import { FormEventHandler, useEffect, useState } from 'react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { Hex, encodeFunctionData } from 'viem';
import { Layout } from './components';
import { ContractType, contracts } from './constants';
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
  const { read: readEntryPoint, write: writeEntryPoint } = useContract(ContractType.ENTRY_POINT);
  const { address } = useAccount();
  const [isTransfering, setTransfering] = useState(false);
  const [transferAddress, setTransferAddress] = useState('');
  const { signMessageAsync } = useSignMessage();

  const { getBytecode } = usePublicClient();

  const flagOwner = useQuery<string, Error, string>(['owner'], {
    queryFn: () => read.flagHolder() as Promise<string>,
    refetchInterval: 5000
  });

  const capturaFlagMutation = useMutation({
    // mutationFn: () => write.captureFlag(),
    mutationFn: async () => {
      if (!address) return;

      const initCode = getInitCode(address);

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const senderAddress = await getSenderAddress(publicClient as any, {
        initCode,
        entryPoint: ENTRY_POINT_ADDRESS
      });

      const call = encodeFunctionData({
        abi: contracts[ContractType.CTF].abi,
        functionName: 'captureFlag',
        args: []
      });

      const callData = encodeFunctionData({
        abi: [
          {
            inputs: [
              { name: 'dest', type: 'address' },
              { name: 'value', type: 'uint256' },
              { name: 'func', type: 'bytes' }
            ],
            name: 'execute',
            outputs: [],
            stateMutability: 'nonpayable',
            type: 'function'
          }
        ],
        args: [contracts[ContractType.CTF].address, 0n, call]
      });

      const gasPrice = await bundlerClient.getUserOperationGasPrice();

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const nonce = await getAccountNonce(publicClient as any, {
        sender: senderAddress,
        entryPoint: ENTRY_POINT_ADDRESS
      });

      const byteCode = await getBytecode({ address: senderAddress });

      const userOperation = {
        sender: senderAddress,
        nonce,
        initCode: byteCode?.toString() ? '0x' : initCode,
        callData,
        maxFeePerGas: gasPrice.fast.maxFeePerGas,
        maxPriorityFeePerGas: gasPrice.fast.maxPriorityFeePerGas,
        // dummy signature
        signature:
          '0xa15569dd8f8324dbeabf8073fdec36d4b754f53ce5901e283c6de79af177dc94557fa3c9922cd7af2a96ca94402d35c39f266925ee6407aeb32b31d76978d4ba1c' as Hex
      };

      const finalUserOperation: UserOperation = {
        ...userOperation,
        callGasLimit: 50305n,
        verificationGasLimit: 80565n,
        preVerificationGas: 56135n,
        paymasterAndData: '0x'
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

      await writeEntryPoint.depositTo([senderAddress], { value: 100000000000000n });

      const signature = await signMessageAsync({
        message: {
          /** Raw data representation of the message. */
          raw: getUserOperationHash({
            userOperation: finalUserOperation,
            chainId: L2_CHAIN_CONFIG.id,
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

      console.log(`UserOperation included: https://goerli-optimism.etherscan.io/tx/${txHash}`);
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
          // disabled={flagOwner.isLoading || isCurrentOwner || transferFlagMutation.isLoading}
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
