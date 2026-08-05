import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  turbopack: {
    // This app sits inside the bob monorepo, which has its own lockfile at the
    // repo root. Without this, Next infers the wrong workspace root.
    root: import.meta.dirname,
  },
  // Neither section has an index page. Proxied traffic from docs.gobob.xyz is
  // already rewritten straight to the overview (see docs/vercel.json); these
  // cover anyone hitting this deployment directly.
  async redirects() {
    return [
      { source: '/', destination: '/gateway/overview', permanent: false },
      { source: '/gateway', destination: '/gateway/overview', permanent: false },
      { source: '/api-reference', destination: '/api-reference/overview', permanent: false },
      // /gateway/strategies was published under Mintlify and has been removed;
      // send existing links and bookmarks to the integration guide.
      { source: '/gateway/strategies', destination: '/gateway/integration', permanent: true },
    ];
  },
};

export default withMDX(config);
