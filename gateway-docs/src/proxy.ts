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

export default function proxy(request: NextRequest) {
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

// Docs pages only: with a root `docsRoute` the patterns above would otherwise
// also rewrite `llms.txt`, assets and API routes.
export const config = {
  matcher: ['/gateway/:path*', '/api-reference/:path*'],
};
