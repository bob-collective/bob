# BOB Gateway Docs

The Gateway developer docs — the `/gateway/*` and `/api-reference/*` sections of
docs.gobob.xyz. Built with [Fumadocs](https://fumadocs.dev) on Next.js.

docs.gobob.xyz itself is the Docusaurus app in `docs/`; it proxies those two
path prefixes to this app's deployment via its `vercel.json` rewrites. This
replaced the previous Mintlify-hosted site.

## Develop

```shell
pnpm install
pnpm dev          # http://localhost:3000
```

`dev` and `build` both regenerate the API reference from the OpenAPI spec first.

## Layout

| Path | Purpose |
|------|---------|
| `content/docs/gateway/` | The hand-written guides |
| `content/docs/api-reference/` | `overview.mdx`, plus `v3/` `v4/` generated from the spec |
| `openapi.json` | The Gateway API spec |
| `src/components/` | MDX component overrides and the live routes table |
| `src/lib/source.ts` | Content source and page tree |

Sidebar order and grouping come from the `meta.json` files. The sidebar and
sitemap target V4. V3 is built and reachable by URL for existing integrations.
V1 and V2 have been removed from the spec and no longer generate pages.

`content/docs/gateway/supported-routes.mdx` renders a live table of routes
fetched from the Gateway API at runtime; the component behind it is
`src/components/supported-routes.tsx`.

## API reference

Generated from `openapi.json`, which is owned by the backend — refresh it with
`make openapi` from the repo root rather than editing it by hand. Changes to it
on master trigger the `openapi-docs-sync` workflow.

```shell
pnpm openapi      # regenerate the reference pages; runs automatically on dev/build
```

The generated `v3/` `v4/` directories are gitignored, and are cleared
before each run so a renamed or removed operation cannot leave a stale page
behind.

## Deployment

Deployed as its own Vercel project with the root directory set to
`gateway-docs`. Both prefixes are proxied from the Docusaurus project's
`docs/vercel.json`, along with `/_next/*` and `/api/search` — without those two
the site would 404 on its own assets and search.
