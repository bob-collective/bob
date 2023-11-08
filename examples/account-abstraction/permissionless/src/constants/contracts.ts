import { CTFAbi } from '../contracts/abi/CTF.abi';

enum ContractType {
  CTF = 'CTF'
}

const contracts = {
  [ContractType.CTF]: {
    address: '0x4478eE0Fd9F054ADBb422c55Beb21DF2bcCe71C8',
    // address: '0x7349289C7C4097D82d670Df784Fec290cB22CCaf',
    // address: '0x587255805b6bcb4Eb34a811AaFb2aa65ef8aA72b',
    abi: CTFAbi
  }
} as const;

export { ContractType, contracts };
