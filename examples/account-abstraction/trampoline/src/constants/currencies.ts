enum Erc20CurrencyTicker {
  ZBTC = 'ZBTC',
  USDT = 'USDT',
  USDC = 'USDC'
}

type CurrencyTicker = keyof typeof Erc20CurrencyTicker | BitcoinTicker;

interface CurrencyBase {
  ticker: string;
  name: string;
  decimals: number;
}

interface Erc20Currency extends CurrencyBase {
  ticker: Erc20CurrencyTicker;
  address: `0x${string}`;
}

type BitcoinTicker = 'BTC';

const Bitcoin = {
  ticker: 'BTC' as BitcoinTicker,
  name: 'Bitcoin',
  decimals: 8
} as const;

type BitcoinCurrency = typeof Bitcoin;

type Currency = Erc20Currency | BitcoinCurrency;

const Erc20Currencies: {
  [ticker in Erc20CurrencyTicker]: Erc20Currency;
} = {
  [Erc20CurrencyTicker.ZBTC]: {
    ticker: Erc20CurrencyTicker.ZBTC,
    name: 'zBTC',
    decimals: 8,
    address: '0x4f01078121e90915F9f1448DE4b3C2515B5e2F3B'
  },
  [Erc20CurrencyTicker.USDT]: {
    ticker: Erc20CurrencyTicker.USDT,
    name: 'Tether USD',
    decimals: 6,
    address: '0x3c252953224948E441aAfdE7b391685201ccd3bC'
  },
  [Erc20CurrencyTicker.USDC]: {
    ticker: Erc20CurrencyTicker.USDC,
    name: 'Circle USD',
    decimals: 6,
    address: '0xfecc3f37038999ede8e58a3c9e5b0e9a16e7d5bc'
  }
};

const currencies = {
  ...Erc20Currencies,
  BTC: Bitcoin
};

export { Erc20Currencies, Erc20CurrencyTicker, currencies, Bitcoin };
export type { Erc20Currency, Currency, BitcoinCurrency, BitcoinTicker, CurrencyTicker };
