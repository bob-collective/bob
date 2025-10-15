import { LayerZeroMessageWallet } from '../types';

export const getCrossChainStatus = (item: LayerZeroMessageWallet) => {
    return item.source.status === 'WAITING'
        ? 'source-pending'
        : item.source.status === 'SIMULATION_REVERTED'
          ? 'source-failed'
          : item.source.status === 'SUCCEEDED' && item.destination.status === 'WAITING'
            ? 'destination-pending'
            : item.source.status === 'SUCCEEDED' && item.destination.status === 'SUCCEEDED'
              ? 'destination-confirmed'
              : item.source.status === 'SUCCEEDED' && item.destination.status === 'SIMULATION_REVERTED'
                ? 'destination-failed'
                : 'unknown';
};
