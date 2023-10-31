import { getContract } from 'viem';
import { usePublicClient, useWalletClient } from 'wagmi';
import { contracts, ContractType } from '../constants';
import { useMemo } from 'react';

// Wrapper around ethers Contract to automatically get contract types
// with read and write objects automatically constructed from provider and signer.
const useContract = (contractType: ContractType) => {
  const publicClient = usePublicClient();
  const { data: walletClient } = useWalletClient();

  return useMemo(() => {
    const { address, abi } = contracts[contractType];
    return getContract({
      address,
      abi,
      publicClient,
      walletClient: walletClient ?? undefined
    });
  }, [walletClient, publicClient, contractType]);
};

export { useContract };
