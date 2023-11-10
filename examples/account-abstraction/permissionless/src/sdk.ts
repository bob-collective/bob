import { bundlerActions } from 'permissionless';
import { pimlicoBundlerActions } from 'permissionless/actions/pimlico';
import { concat, createClient, createPublicClient, encodeFunctionData, http } from 'viem';
import { L2_CHAIN_CONFIG } from './connectors/wagmi-connectors';
import { SIMPLE_ACCOUNT_FACTORY_ADDRESS } from './constants/erc4337';

// const chain = optimismGoerli;
const chain = L2_CHAIN_CONFIG;
// const apiKey = import.meta.env.VITE_PIMLICO_API_KEY; // REPLACE THIS

// CREATE THE CLIENTS
export const publicClient = createPublicClient({
  transport: http('https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz'),
  chain
});

export const bundlerClient = createClient({
  transport: http(`http://localhost:3000`),
  // transport: http(`https://api.pimlico.io/v1/${'optimism-goerli'}/rpc?apikey=${apiKey}`),
  chain
})
  .extend(bundlerActions)
  .extend(pimlicoBundlerActions);

// export const paymasterClient = createClient({
//   // ⚠️ using v2 of the API ⚠️
//   transport: http(`https://api.pimlico.io/v2/${'optimism-goerli'}/rpc?apikey=${apiKey}`),
//   chain
// }).extend(pimlicoPaymasterActions);

export const getInitCode = (ownerAddress: `0x${string}`) => {
  return concat([
    SIMPLE_ACCOUNT_FACTORY_ADDRESS,
    encodeFunctionData({
      abi: [
        {
          inputs: [
            { name: 'owner', type: 'address' },
            { name: 'salt', type: 'uint256' }
          ],
          name: 'createAccount',
          outputs: [{ name: 'ret', type: 'address' }],
          stateMutability: 'nonpayable',
          type: 'function'
        }
      ],
      args: [ownerAddress, 0n]
    })
  ]);
};
