import { CTFAbi } from '../contracts/abi/CTF.abi';
import { ERC20Abi } from '../contracts/abi/ERC20.abi';
import { Erc20Currencies, Erc20CurrencyTicker } from './currencies';

enum ContractType {
  ZBTC = 'ZBTC',
  USDT = 'USDT',
  USDC = 'USDC',
  CTF = 'CTF'
}

const contracts = {
  // Automatically adds all ERC20 currencies contracts here.
  ...Object.entries(Erc20Currencies).reduce(
    (result, [key, value]) => ({ ...result, [key as ContractType]: { ...value, abi: ERC20Abi } }),
    {} as { [ticker in Erc20CurrencyTicker]: { abi: typeof ERC20Abi; address: `0x${string}` } }
  ),
  [ContractType.CTF]: {
    address: '0x587255805b6bcb4Eb34a811AaFb2aa65ef8aA72b',
    abi: CTFAbi
  }
} as const;

export { ContractType, contracts };
