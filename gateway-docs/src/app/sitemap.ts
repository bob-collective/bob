import type { MetadataRoute } from 'next';
import { source } from '@/lib/source';
import { siteUrl } from '@/lib/shared';

export const revalidate = false;

/**
 * URLs are emitted against docs.gobob.xyz rather than this deployment's own
 * domain, because that is where the pages are actually served from.
 *
 * v1 and v2 of the API reference are deprecated and hidden from the sidebar, so
 * they are left out here too -- they stay reachable by URL, but there is no
 * reason to invite crawlers to index them over v3.
 */
const DEPRECATED = ['/api-reference/v1/', '/api-reference/v2/'];

export default function sitemap(): MetadataRoute.Sitemap {
  return source
    .getPages()
    .filter((page) => !DEPRECATED.some((prefix) => page.url.startsWith(prefix)))
    .map((page) => ({
      url: new URL(page.url, siteUrl).toString(),
      changeFrequency: 'weekly',
      priority: page.url === '/gateway/overview' ? 1 : 0.7,
    }));
}
