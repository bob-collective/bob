/**
 * Code only the async execute paths of the client use (`executeQuote` for offramp and tokenSwap,
 * `executeStrategy`): ABIs, the AllowanceHolder check, gas estimation and Tron address handling.
 * The client loads it with `import()` on first use, so an app that only builds a `GatewaySDK` and
 * fetches quotes, routes or orders does not ship it in its initial chunk.
 */
export { erc20Abi } from 'viem';
export { oftApprovalRequiredAbi, strategyCaller, USDTApproveAbi } from './abi';
export { assertAllowanceHolderSpender } from './allowance-holder';
export { estimateGasWithBuffer, isValidTronAddress, tronAddressToHex } from './utils';
