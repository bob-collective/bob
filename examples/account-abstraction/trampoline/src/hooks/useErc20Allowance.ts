import { useState, useCallback, useEffect } from 'react';
import { ContractType, Erc20CurrencyTicker, UINT_256_MAX, contracts } from '../constants';
import { useAccount, usePublicClient } from 'wagmi';
import { useContract } from './useContract';

// Helper hook to check erc20 allowance and provide wrapper that handles allowance;
const useErc20Allowance = (contract: ContractType, ticker: Erc20CurrencyTicker) => {
  const [isErc20TransferApproved, setIsErc20TransferApproved] = useState(false);
  const [contractType, setContractType] = useState(ContractType[ticker]);
  const [isLoading, setLoading] = useState(false);

  useEffect(() => {
    const type = ContractType[ticker];

    if (!type) return;

    setContractType(type);
  }, [ticker]);

  const publicClient = usePublicClient();

  const { address } = useAccount();

  const { read: readErc20Contract, write: writeErc20Contract } = useContract(contractType);

  const fetchAllowance = useCallback(async () => {
    if (readErc20Contract && address) {
      const allowance = await readErc20Contract.allowance([address, '0x18374A837Bb438a7C5F3b47799ceEf0Ee481C866']);
      // If allowance is lower than maximum, then it is considered unallowed and approval tx is required. (to keep poc simple)
      if (allowance < UINT_256_MAX) {
        setIsErc20TransferApproved(false);
      } else {
        setIsErc20TransferApproved(true);
      }
    }
  }, [readErc20Contract, address, contract]);

  useEffect(() => {
    fetchAllowance();
  }, [fetchAllowance]);

  const wrapInErc20ApprovalTx = useCallback(
    async (subroutine: () => void) => {
      if (!writeErc20Contract || isErc20TransferApproved) {
        return subroutine();
      }

      setLoading(true);

      try {
        const txHash = await writeErc20Contract.approve(['0x18374A837Bb438a7C5F3b47799ceEf0Ee481C866', UINT_256_MAX]);
        await publicClient.waitForTransactionReceipt({ hash: txHash });
      } catch (e) {
        setLoading(false);
      }

      fetchAllowance();
      setLoading(false);

      return subroutine();
    },
    [fetchAllowance, publicClient, writeErc20Contract, isErc20TransferApproved, contract]
  );

  return { isLoading, isAllowed: isErc20TransferApproved, wrapInErc20ApprovalTx };
};

export { useErc20Allowance };
