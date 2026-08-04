# BOB Gateway Docs

The Gateway developer docs — the `/gateway/*` and `/api-reference/*` sections of
docs.gobob.xyz. Built with [Fumadocs](https://fumadocs.dev) on Next.js.

This replaces the previous Mintlify-hosted site. The main docs.gobob.xyz site is
still the Docusaurus app in `docs/`, which proxies these two path prefixes here
via its `vercel.json` rewrites.

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
| `content/docs/api-reference/` | `overview.mdx`, plus `v1/` `v2/` `v3/` generated from the spec |
| `src/components/` | MDX component overrides and the live routes table |
| `src/lib/source.ts` | Content source and page tree |
| `scripts/` | Generation and migration scripts |

Sidebar order and grouping come from the `meta.json` files. `v1/` and `v2/` are
built and reachable by URL but deliberately left out of
`content/docs/api-reference/meta.json`, so they stay out of the sidebar.

## API reference

Generated from `docs/gateway/api-reference/openapi.json` — which is owned by the
backend and refreshed by the `openapi-docs-sync` workflow. Do not edit the spec
here.

```shell
pnpm openapi      # regenerate; runs automatically on dev/build
```

The generated `v1/` `v2/` `v3/` directories are gitignored.

## Migrating content from Mintlify

`scripts/migrate-from-mintlify.mjs` ports `docs/gateway/**/*.mdx` into
`content/docs/`, mapping Mintlify components onto their fumadocs equivalents. It
is re-runnable, so it can pick up any late edits to the Mintlify source before
cutover:

```shell
pnpm migrate:mintlify
```

`gateway/supported-routes.mdx` is skipped — Mintlify allowed inline React with
hooks in MDX, so that page's route table was extracted to
`src/components/supported-routes.tsx` and is maintained by hand.

Once Mintlify is switched off, delete the script and `docs/gateway/`, and edit
`content/docs/` directly.
