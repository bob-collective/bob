import type { MetadataRoute } from 'next';
import { source } from '@/lib/source';
import { siteUrl } from '@/lib/shared';

export const revalidate = false;

/**
 * URLs are emitted against docs.gobob.xyz rather than this deployment's own
 * domain, because that is where the pages are actually served from.
 *
 * Index the current V4 API reference. V3 remains reachable for existing
 * integrations but is omitted from the sidebar and sitemap.
 */
const LEGACY_REFERENCE = ['/api-reference/v1/', '/api-reference/v2/', '/api-reference/v3/'];

export default function sitemap(): MetadataRoute.Sitemap {
  return source
    .getPages()
    .filter((page) => !LEGACY_REFERENCE.some((prefix) => page.url.startsWith(prefix)))
    .map((page) => ({
      url: new URL(page.url, siteUrl).toString(),
      changeFrequency: 'weekly',
      priority: page.url === '/gateway/overview' ? 1 : 0.7,
    }));
}
