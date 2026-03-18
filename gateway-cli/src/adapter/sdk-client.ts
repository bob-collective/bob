import { GatewaySDK } from '@gobob/bob-sdk';

const DEFAULT_API_URL = 'https://gateway-api-mainnet.gobob.xyz';

let _instance: InstanceType<typeof GatewaySDK> | null = null;
let _currentUrl: string | undefined;

export function createSdkClient(apiUrl?: string): InstanceType<typeof GatewaySDK> {
  const url = apiUrl || DEFAULT_API_URL;
  if (!_instance || _currentUrl !== url) {
    _instance = new GatewaySDK(url);
    _currentUrl = url;
  }
  return _instance;
}

export function resetSdkClient(): void {
  _instance = null;
  _currentUrl = undefined;
}
