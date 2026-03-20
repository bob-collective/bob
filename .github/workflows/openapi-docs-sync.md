---
description: Review merged OpenAPI spec changes and open a PR with relevant docs updates.
on:
  push:
    branches:
      - master
    paths:
      - docs/gateway/api-reference/openapi.json
engine: claude
permissions: read-all
tools:
  github:
    toolsets: [default]
safe-outputs:
  create-pull-request:
    max: 1
  noop:
  missing-tool:
    create-issue: true
---

# OpenAPI Docs Sync

You are a repository-aware coding agent working in this repository.

## Mission

When this workflow is triggered by a push to master that includes changes to docs/gateway/api-reference/openapi.json, review the OpenAPI spec diff and create a focused documentation update pull request if documentation changes are needed.

## Required Process

1. Confirm the changed file list for this push and verify docs/gateway/api-reference/openapi.json is included.
2. Compute and inspect the spec diff for this push (prefer the push range: github.event.before..github.sha).
3. Determine what changed semantically in the API surface (endpoints, params, request/response shapes, error codes, behavior notes, examples).
4. Find and update relevant documentation files so docs remain accurate and consistent with the spec.
5. Keep updates minimal and targeted to the OpenAPI changes in this push.
6. Validate documentation quality (links, formatting, and internal consistency).

## Scope Rules

- Update only documentation and related generated docs artifacts when needed.
- Do not modify contracts, SDK, relayer, or other unrelated source code.
- Do not revert or alter unrelated local changes.
- Prefer concise wording and preserve existing documentation style.

## Pull Request Requirements

When documentation updates are necessary:

1. Create a branch.
2. Commit only relevant documentation updates.
3. Open one pull request using safe output `create-pull-request`.
4. Use this PR title format: `[openapi-docs] Sync docs for OpenAPI updates (<short date>)`.
5. In the PR body, include:
   - Summary of OpenAPI changes detected in this push.
   - Files updated and why.
   - Any assumptions or follow-up items.

## No-Change Path

If the OpenAPI diff does not require documentation edits (for example, non-user-visible metadata-only changes), call safe output `noop` with a clear explanation of what you checked and why no PR was created.
