import { describe, it, expect } from 'vitest';
import { createSdkClient } from '../../src/adapter/sdk-client';

describe('createSdkClient', () => {
  it('creates a GatewaySDK instance with default URL', () => {
    const sdk = createSdkClient();
    expect(sdk).toBeDefined();
    expect(sdk.getRoutes).toBeTypeOf('function');
    expect(sdk.getQuote).toBeTypeOf('function');
  });

  it('creates a GatewaySDK instance with custom URL', () => {
    const sdk = createSdkClient('https://gateway-api-staging.gobob.xyz');
    expect(sdk).toBeDefined();
  });

  it('returns same instance for same URL', () => {
    const a = createSdkClient('https://example.com');
    const b = createSdkClient('https://example.com');
    expect(a).toBe(b);
  });

  it('returns different instance for different URL', () => {
    const a = createSdkClient('https://a.com');
    const b = createSdkClient('https://b.com');
    expect(a).not.toBe(b);
  });
});
