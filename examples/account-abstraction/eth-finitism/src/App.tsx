import { createWeb3Modal } from '@web3modal/wagmi/react';
import { useAccount, useMutation, useQuery, useSignTypedData } from 'wagmi';
import { L2_CHAIN_CONFIG, L2_PROJECT_ID, config } from './connectors/wagmi-connectors';

import { CTA, Card, Flex, H1, Input, P, Span } from '@interlay/ui';
import { FormEventHandler, useEffect, useState } from 'react';
import Jazzicon, { jsNumberForAddress } from 'react-jazzicon';
import truncateEthAddress from 'truncate-eth-address';
import { encodeFunctionData } from 'viem';
import { Layout } from './components';
import { ContractType, contracts } from './constants';
import { useAccountAbstraction } from './context/AuthContext';
import { useContract } from './hooks/useContract';
import { ENTRY_POINT_ADDRESS } from './constants/erc4337';
import { BigNumberish } from 'ethers';

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
  const { accountAPI, bundlerClient } = useAccountAbstraction();
  const { signTypedDataAsync } = useSignTypedData();

  const flagOwner = useQuery<string, Error, string>(['owner'], {
    queryFn: () => read.flagHolder() as Promise<string>,
    refetchInterval: 5000,
    enabled: false
  });

  const capturaFlagMutation = useMutation({
    // mutationFn: () => write.captureFlag(),
    mutationFn: async () => {
      if (!address || !accountAPI) return;

      const data = encodeFunctionData({
        abi: contracts[ContractType.CTF].abi,
        functionName: 'captureFlag',
        args: []
      });

      const op = await accountAPI.createUnsignedUserOp({
        target: contracts[ContractType.CTF].address,
        value: 0,
        data,
        maxFeePerGas: 0x6507a5d0,
        maxPriorityFeePerGas: 0x6507a5c0
      });

      const userOpTypedData = {
        types: {
          EIP712Domain: [
            { name: 'name', type: 'string' },
            { name: 'chainId', type: 'uint256' },
            { name: 'verifyingContract', type: 'address' }
          ],
          UserOperation: [
            { name: 'sender', type: 'address' },
            { name: 'nonce', type: 'uint256' },
            { name: 'initCode', type: 'bytes' },
            { name: 'callGasLimit', type: 'uint256' },
            { name: 'verificationGasLimit', type: 'uint256' },
            { name: 'preVerificationGas', type: 'uint256' },
            { name: 'paymasterAndData', type: 'bytes' },
            { name: 'signature', type: 'bytes' }
          ]
        },
        domain: {
          name: 'EntryPoint',
          chainId: L2_CHAIN_CONFIG.id,
          verifyingContract: ENTRY_POINT_ADDRESS
        },
        primaryType: 'UserOperation',
        message: {
          sender: await op.sender,
          nonce: ((await op.nonce) as BigNumberish).toBigInt(),
          initCode: await op.initCode,
          callGasLimit: ((await op.callGasLimit) as BigNumberish).toBigInt(),
          verificationGasLimit: ((await op.verificationGasLimit) as BigNumberish).toBigInt(),
          preVerificationGas: await op.preVerificationGas,
          paymasterAndData: await op.paymasterAndData,
          signature:
            '0xa15569dd8f8324dbeabf8073fdec36d4b754f53ce5901e283c6de79af177dc94557fa3c9922cd7af2a96ca94402d35c39f266925ee6407aeb32b31d76978d4ba1c'
        }
      };

      const signature = await signTypedDataAsync(userOpTypedData as any);

      // console.log(signature);
      // const a = await accountAPI.createSignedUserOp({
      //   target: contracts[ContractType.CTF].address,
      //   value: 0,
      //   data,
      //   maxFeePerGas: 0x6507a5d0,
      //   maxPriorityFeePerGas: 0x6507a5c0
      // });

      // console.log(await a.signature);

      op.signature = signature;

      // await op.signature;

      await bundlerClient?.sendUserOpToBundler(op);
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
