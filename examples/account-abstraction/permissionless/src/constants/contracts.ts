import { CTFAbi } from '../contracts/abi/CTF.abi';
import { entryPointAbi } from '../contracts/abi/EntryPoint.abi';
import { ENTRY_POINT_ADDRESS } from './erc4337';

enum ContractType {
  CTF = 'CTF',
  ENTRY_POINT = 'ENTRY_POINT'
}

const contracts = {
  [ContractType.CTF]: {
    // address: '0x4478eE0Fd9F054ADBb422c55Beb21DF2bcCe71C8',
    // address: '0x7349289C7C4097D82d670Df784Fec290cB22CCaf',
    address: '0x587255805b6bcb4Eb34a811AaFb2aa65ef8aA72b',
    abi: CTFAbi
  },
  [ContractType.ENTRY_POINT]: {
    address: ENTRY_POINT_ADDRESS,
    abi: entryPointAbi
  }
} as const;

export { ContractType, contracts };
