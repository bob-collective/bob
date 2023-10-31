import { CTA, CTAProps } from '@interlay/ui';
import { useWeb3Modal } from '@web3modal/wagmi/react';
import { useAccount } from 'wagmi';

type AuthCTAProps = CTAProps;

const AuthCTA = ({ onPress, onClick, disabled, children, type, ...props }: AuthCTAProps) => {
  const { address } = useAccount();
  const { open } = useWeb3Modal();

  const authProps = address
    ? { onPress, onClick, disabled, children, type, ...props }
    : { onPress: () => open(), children: 'Connect Wallet', ...props };

  return <CTA {...authProps} />;
};

export { AuthCTA };
