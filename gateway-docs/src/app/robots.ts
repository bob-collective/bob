import type { MetadataRoute } from 'next';
import { siteUrl } from '@/lib/shared';

export const revalidate = false;

export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: '*',
      allow: '/',
    },
    // Public location, which is where the proxy exposes it.
    sitemap: `${siteUrl}/gateway/sitemap.xml`,
  };
}
