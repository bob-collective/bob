import { CTFAbi } from '../contracts/abi/CTF.abi';

enum ContractType {
  CTF = 'CTF'
}

const contracts = {
  [ContractType.CTF]: {
    address: '0x7349289C7C4097D82d670Df784Fec290cB22CCaf',
    abi: CTFAbi
  }
} as const;

export { ContractType, contracts };
