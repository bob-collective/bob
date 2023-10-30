import { CTA } from '@interlay/ui';
import { useState } from 'react';
import { useAccount } from 'wagmi';

const EthFaucet = () => {
  const [isLoading, setLoading] = useState(false);

  const { address } = useAccount();

  const faucetEndpoint = `https://faucetl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/drip/${address}`;

  const handleCallFaucet = async () => {
    setLoading(true);
    try {
      await fetch(faucetEndpoint, {
        method: 'POST'
      });
    } catch (e) {
      console.log(`ETH faucet failed: ${e}`);
      setLoading(false);
    }
    setLoading(false);
  };

  return (
    <CTA loading={isLoading} onPress={() => handleCallFaucet()} size='small'>
      Get ETH
    </CTA>
  );
};

export { EthFaucet };
