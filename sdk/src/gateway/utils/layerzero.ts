import { LayerZeroMessageWallet } from '../types';
import { CrossChainOrderStatus } from '../types/crosschain-swap';

export const getCrossChainStatus = (item: LayerZeroMessageWallet): CrossChainOrderStatus => {
    const { source, destination } = item;

    // Handle source transaction states first
    if (source.status === 'WAITING') {
        return 'source-pending';
    }

    if (source.status === 'SIMULATION_REVERTED') {
        return 'source-failed';
    }

    // If source succeeded, check destination status
    if (source.status === 'SUCCEEDED') {
        switch (destination.status) {
            case 'WAITING':
                return 'destination-pending';
            case 'SUCCEEDED':
                return 'destination-confirmed';
            case 'SIMULATION_REVERTED':
                return 'destination-failed';
            default:
                return 'unknown';
        }
    }

    // Any other source status is unknown
    return 'unknown';
};
