export const appName = 'BOB Gateway';

// Docs are served from the root of this app so the public URLs match what
// Mintlify served: /gateway/* and /api-reference/*. docs.gobob.xyz proxies
// those two prefixes here (see docs/vercel.json).
export const docsRoute = '/';
export const docsImageRoute = '/og';
export const docsContentRoute = '/llms.mdx';

export const gitConfig = {
  user: 'bob-collective',
  repo: 'bob',
  branch: 'master',
  // this app lives in a subdirectory of the monorepo
  contentPath: 'gateway-docs/content/docs',
};

export const siteUrl = 'https://docs.gobob.xyz';

export const links = {
  homepage: 'https://gobob.xyz',
  bobDocs: 'https://docs.gobob.xyz',
  launchApp: 'https://app.gobob.xyz/swap',
  discord: 'https://discord.gg/gobob',
  github: 'https://github.com/bob-collective',
  analytics: 'https://dune.com/bob_collective/gateway',
  x: 'https://x.com/build_on_bob',
};
