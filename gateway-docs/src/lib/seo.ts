import type { source } from '@/lib/source';

type Page = (typeof source)['$inferPage'];

/**
 * Meta description for a page whose front matter has none. The generated API
 * reference pages have no `description` (the OpenAPI operations only carry a
 * summary), and a missing one leaves search engines to pick a snippet.
 *
 * Kept out of the front matter on purpose: that would also render it as a
 * subtitle under the page heading.
 */
export function describePage(page: Page): string | undefined {
  if (!('_openapi' in page.data)) return undefined;

  // Summaries read like "Get a gateway quote (V4 — `refundAddress` is optional
  // here, ...)": keep the lead and drop the parenthetical and any markdown.
  const summary = page.data.title
    .replace(/\s+/g, ' ')
    .replace(/\s*\(.*$/s, '')
    .replace(/`/g, '')
    .replace(/\.$/, '')
    .trim();
  const version = page.slugs.find((slug) => /^v\d+$/.test(slug))?.toUpperCase();

  return `${summary}. BOB Gateway${version ? ` ${version}` : ''} API reference with request parameters, response schema and examples.`;
}
