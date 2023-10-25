import { CTFAbi } from '../contracts/abi/CTF.abi';

enum ContractType {
  CTF = 'CTF'
}

const contracts = {
  [ContractType.CTF]: {
    address: '0x587255805b6bcb4Eb34a811AaFb2aa65ef8aA72b',
    abi: CTFAbi
  }
} as const;

export { ContractType, contracts };
