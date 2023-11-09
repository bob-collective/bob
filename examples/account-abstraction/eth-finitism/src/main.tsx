import { InterlayUIProvider } from '@interlay/system';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { WagmiConfig } from 'wagmi';
import App from './App';
import { config } from './connectors/wagmi-connectors';
import './index.css';
import '@interlay/theme/dist/bob.css';
import { CSSReset } from '@interlay/ui';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { AccountAbstractionProvider } from './context/AuthContext';

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <WagmiConfig config={config}>
        <AccountAbstractionProvider>
          <InterlayUIProvider>
            <CSSReset />
            <App />
          </InterlayUIProvider>
        </AccountAbstractionProvider>
      </WagmiConfig>
    </QueryClientProvider>
  </React.StrictMode>
);
