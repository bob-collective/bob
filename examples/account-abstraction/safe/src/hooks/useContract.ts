import { ethers } from 'ethers';
import { useEffect, useState } from 'react';
import { ContractType, contracts } from '../constants';
import { useAccountAbstraction } from '../context/AuthContext';

function useContract(contractType: ContractType) {
  const [contract, setContract] = useState<ethers.Contract | null>(null);
  const { web3Provider } = useAccountAbstraction();

  useEffect(() => {
    // Ensure you have an Ethereum provider set up, e.g., with MetaMask
    if (window.ethereum && web3Provider) {
      console.log(web3Provider.network);
      const signer = web3Provider.getSigner();

      const contractObj = contracts[contractType];

      const contractInit = new ethers.Contract(contractObj.address, contractObj.abi, signer);
      setContract(contractInit);
    } else {
      console.error('Ethereum provider not found. Make sure you have MetaMask or a similar provider installed.');
    }
  }, [contractType, web3Provider]);

  // Read function
  const read = async (methodName: string, ...args: unknown[]) => {
    if (!contract) {
      console.error('Contract not initialized. Check your Ethereum provider.');
      return null;
    }

    try {
      const result = await contract[methodName](...args);
      return result;
    } catch (error) {
      console.error(`Error calling read function ${methodName}:`, error);
      return null;
    }
  };

  // Write function (for transactions)
  const write = async (methodName: string, ...args: unknown[]) => {
    if (!contract) {
      console.error('Contract not initialized. Check your Ethereum provider.');
      return null;
    }

    try {
      const transaction = await contract[methodName](...args);
      await transaction.wait();
      return transaction;
    } catch (error) {
      console.error(`Error calling write function ${methodName}:`, error);
      return null;
    }
  };

  return { contract, read, write };
}

export { useContract };
