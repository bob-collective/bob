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

function route(request: NextRequest) {
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
 * Only that exact match counts as direct, so a missing or different value
 * leaves the page indexable and docs.gobob.xyz can never be noindexed by it.
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

// Docs pages only: with a root `docsRoute` the patterns above would otherwise
// also rewrite `llms.txt`, assets and API routes.
export const config = {
  matcher: ['/gateway/:path*', '/api-reference/:path*'],
};
