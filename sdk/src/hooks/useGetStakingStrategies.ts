import { useQuery } from '@tanstack/react-query';

import { GatewaySDK } from '../gateway';

const ONE_HOUR = 60 * 60 * 1000;

const useGetStakingStrategies = (gatewaySDK: GatewaySDK) => {
    return useQuery({
        queryKey: ['staking-strategies'],
        refetchOnWindowFocus: false,
        refetchOnMount: false,
        gcTime: ONE_HOUR,
        queryFn: () => gatewaySDK.getStrategies(),
    });
};

export { useGetStakingStrategies };
