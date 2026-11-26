import { NextRequest, NextResponse } from 'next/server';
import { isMarkdownPreferred, rewritePath } from 'fumadocs-core/negotiation';
import { docsContentRoute, docsRoute } from '@/lib/shared';

// Docs are served from the root, so `docsRoute` is `/`; without trimming it the
// patterns start with `//` and never match.
const docsPrefix = docsRoute.replace(/\/$/, '');

const { rewrite: rewriteDocs } = rewritePath(
  `${docsPrefix}{/*path}`,
  `${docsContentRoute}{/*path}/content.md`,
);
const { rewrite: rewriteSuffix } = rewritePath(
  `${docsPrefix}{/*path}.md`,
  `${docsContentRoute}{/*path}/content.md`,
);

// The matcher also runs this proxy on non-docs paths, only to decide the
// header. Those must not go through the docs rewrite logic.
function isDocsPath(pathname: string): boolean {
  return /^\/(gateway|api-reference)(\/|$)/.test(pathname);
}

function route(request: NextRequest) {
  if (!isDocsPath(request.nextUrl.pathname)) return NextResponse.next();

  const result = rewriteSuffix(request.nextUrl.pathname);
  if (result) {
    return NextResponse.rewrite(new URL(result, request.nextUrl));
  }

  // A path with a file extension is a static file from `public/` (e.g.
  // `openapi.json`), not a page, whatever the client's `Accept` prefers.
  const isFile = /\.[^/]+$/.test(request.nextUrl.pathname);
  if (!isFile && isMarkdownPreferred(request)) {
    const result = rewriteDocs(request.nextUrl.pathname);

    if (result) {
      return NextResponse.rewrite(new URL(result, request.nextUrl), {
        // this URL has two representations, selected by `Accept`
        headers: { Vary: 'Accept' },
      });
    }
  }

  return NextResponse.next();
}

/**
 * This app is public at its own *.vercel.app address as well as behind
 * docs.gobob.xyz, which rewrites /gateway/* and /api-reference/* here. Only
 * the docs.gobob.xyz copy should be indexed.
 *
 * `Host` is this deployment's host on both kinds of request, so it cannot tell
 * them apart. `x-forwarded-host` can: Vercel sets it to the host the client
 * asked for, overwriting any client-sent value. A proxied request therefore
 * carries `docs.gobob.xyz`, while a direct one carries the same host as `Host`.
 *
 * Only that exact match counts as direct, so a different value leaves the page
 * indexable. A missing value is not "different": Next fills a missing
 * `x-forwarded-host` with `Host` before the proxy runs, so such a request is
 * treated as direct and gets noindex. docs.gobob.xyz therefore stays indexable
 * only because Vercel sets `x-forwarded-host` on external rewrites
 * (https://vercel.com/docs/rewrites, "Identifying the visitor at your origin").
 * Check it with `curl` right after every production deploy.
 */
function isDirectRequest(request: NextRequest): boolean {
  const host = request.headers.get('host');
  const forwardedHost = request.headers.get('x-forwarded-host');

  if (host === null || forwardedHost === null) return false;

  return host.toLowerCase() === forwardedHost.toLowerCase();
}

// A header rather than a redirect, so preview deployments stay viewable, and
// rather than a robots.txt `Disallow`, which would stop crawlers from ever
// reading the noindex.
export default function proxy(request: NextRequest) {
  const response = route(request);

  if (isDirectRequest(request)) {
    response.headers.set('X-Robots-Tag', 'noindex');
  }

  return response;
}

// Docs pages, plus the other crawlable copies of them (Markdown, llms files,
// OG images) so the direct copy is noindexed too. `route` keeps the rewrite
// logic to the docs pages: with a root `docsRoute` its patterns would
// otherwise also rewrite `llms.txt`, assets and API routes.
export const config = {
  matcher: [
    '/gateway/:path*',
    '/api-reference/:path*',
    '/llms.mdx/:path*',
    '/llms.txt',
    '/llms-full.txt',
    '/og/:path*',
  ],
};
