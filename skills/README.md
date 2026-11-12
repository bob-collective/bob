# Agent skills

Skills that teach AI agents to use [BOB Gateway](https://docs.gobob.xyz/gateway/overview):
swap native BTC to and from tokens on EVM chains and Tron, and swap tokens across
those chains. They follow the open [Agent Skills](https://agentskills.io) format, so
they work in Claude Code, Codex, Cursor, Gemini CLI, GitHub Copilot, OpenCode and
other agents that read `SKILL.md` files.

| Skill | Use it to |
|---|---|
| [`gateway-swap`](gateway-swap/SKILL.md) | Quote a swap, create the order, and hand the user what to sign |
| [`gateway-order-status`](gateway-order-status/SKILL.md) | Look up an order or an address's history and explain the status |
| [`gateway-integrate`](gateway-integrate/SKILL.md) | Write or review code that integrates Gateway (SDK, REST API, CLI) |

`gateway-swap` and `gateway-order-status` call the tools of the Gateway MCP server:
`get_routes`, `get_quote`, `create_order`, `register_tx`, `get_order`, `get_orders`
and `get_max_spendable`. The server is a pass-through to the public Gateway V4 API.
`gateway-integrate` works without it.

Neither the server nor the skills hold keys or sign anything. The agent prepares
the order and tells the user what to sign, and the user's own wallet signs and
broadcasts every transaction, token approvals included.

## Install

Install the skills, then connect the MCP server.

### Claude Code

The `bob-gateway` plugin bundles the three skills and the MCP server:

```bash
claude plugin marketplace add bob-collective/bob
claude plugin install bob-gateway@bob
```

The plugin reads two environment variables when Claude Code starts:

| Variable | Default | Purpose |
|---|---|---|
| `GATEWAY_API_KEY` | none | Integrator API key, sent as `Authorization: Bearer <key>` (see [API key](#api-key)) |
| `BOB_GATEWAY_MCP_URL` | `https://mcp.gobob.xyz/mcp` | Point the plugin at another Gateway MCP server |

### Other agents

Install the skills with the [`skills` CLI](https://github.com/vercel-labs/skills),
which knows where each agent looks for them:

```bash
npx skills add bob-collective/bob                # into the current project
npx skills add bob-collective/bob -g -a codex    # for one agent, user-wide
```

Or copy the skill directories into `.agents/skills/` in your project. Codex,
Cursor, Gemini CLI, GitHub Copilot and OpenCode all read that directory.

Then connect the MCP server: streamable HTTP at `https://mcp.gobob.xyz/mcp`, with
an optional `Authorization: Bearer <key>` header. Without a key, leave the header
(or Codex's `--bearer-token-env-var`) out.

**Codex**

```bash
codex mcp add bob-gateway --url https://mcp.gobob.xyz/mcp --bearer-token-env-var GATEWAY_API_KEY
```

**Cursor** — `.cursor/mcp.json` in the project, or `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "bob-gateway": {
      "url": "https://mcp.gobob.xyz/mcp",
      "headers": { "Authorization": "Bearer ${env:GATEWAY_API_KEY}" }
    }
  }
}
```

**Gemini CLI** — `.gemini/settings.json` in the project, or `~/.gemini/settings.json`:

```json
{
  "mcpServers": {
    "bob-gateway": {
      "httpUrl": "https://mcp.gobob.xyz/mcp",
      "headers": { "Authorization": "Bearer ${GATEWAY_API_KEY}" }
    }
  }
}
```

**GitHub Copilot** — `.mcp.json` in the workspace root, or `~/.copilot/mcp-config.json`:

```json
{
  "mcpServers": {
    "bob-gateway": {
      "type": "http",
      "url": "https://mcp.gobob.xyz/mcp",
      "headers": { "Authorization": "Bearer ${GATEWAY_API_KEY}" }
    }
  }
}
```

**Claude Code without the plugin**

```bash
claude mcp add --transport http bob-gateway https://mcp.gobob.xyz/mcp \
  --header "Authorization: Bearer $GATEWAY_API_KEY"
```

Any other agent: add a streamable HTTP MCP server named `bob-gateway` with the same
URL and header.

### API key

`GATEWAY_API_KEY` is an optional 32-character integrator API key. With it, the
gateway applies that integrator's pricing to the orders; every tool works without
it. Request one from gateway@gobob.xyz, and set it in the environment your agent
starts from, never in chat.

`create_order` creates a real order, and `register_tx` broadcasts a Bitcoin
transaction that cannot be undone. Keep both behind your agent's tool-approval
prompt.

## Contributing

- Each skill is a directory with a `SKILL.md` that follows the
  [Agent Skills specification](https://agentskills.io/specification). Its frontmatter
  `name` is the directory name: lowercase letters, digits and hyphens, at most 64
  characters. Its `description`, at most 1024 characters, starts with "Use when" and
  says when to load the skill.
- Keep each skill self-contained: link only to files inside its own directory.
  Installers copy each skill directory on its own.
- Keep the skills agent-neutral. Name MCP tools as the server names them, and give
  no instructions that only one agent understands.
- Skills never handle keys, and never sign or send a transaction. They tell the
  user what to sign.
- The skills are read by agents outside BOB: keep them to public endpoints, tools
  and docs.
- A new skill joins the Claude Code plugin once it is listed under `skills` in
  [`.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json). Bump the
  plugin's `version` there with every change to a skill.

`node .github/scripts/check-skills.mjs` checks the rules above that a script can
check. It also checks the skills against the sources they restate: the endpoints
and error codes in `gateway-docs/openapi.json`, and the AllowanceHolder spenders in
`sdk/src/gateway/allowance-holder.ts`. On every pull request that touches the
skills or those sources, the `Skills` workflow runs it, the Agent Skills reference
validator ([`skills-ref validate`](https://github.com/agentskills/agentskills/tree/main/skills-ref))
on each skill, and `claude plugin validate .` on the marketplace.
