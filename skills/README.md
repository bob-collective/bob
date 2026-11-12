# Agent skills

Skills that teach AI agents to use [BOB Gateway](https://docs.gobob.xyz/gateway/overview):
swap native BTC to and from tokens on EVM chains and Tron, and swap tokens across
those chains.

| Skill | Use it to |
|---|---|
| [`gateway-swap`](gateway-swap/SKILL.md) | Quote a swap, create the order, and hand the user what to sign |
| [`gateway-order-status`](gateway-order-status/SKILL.md) | Look up an order or an address's history and explain the status |
| [`gateway-integrate`](gateway-integrate/SKILL.md) | Write or review code that integrates Gateway (SDK, REST API, CLI) |

`gateway-swap` and `gateway-order-status` call the tools of the Gateway MCP server:
`get_routes`, `get_quote`, `create_order`, `register_tx`, `get_order`, `get_orders`
and `get_max_spendable`. The server is a pass-through to the public Gateway V4 API.
It holds no keys and signs nothing: the user's own wallet signs and broadcasts every
transaction. `gateway-integrate` works without it.

## Install in Claude Code

The `bob-gateway` plugin bundles the three skills and the MCP server:

```bash
claude plugin marketplace add bob-collective/bob
claude plugin install bob-gateway@bob
```

Both settings are environment variables, read when Claude Code starts.

| Variable | Default | Purpose |
|---|---|---|
| `GATEWAY_API_KEY` | none | 32-character integrator API key, sent as `Authorization: Bearer <key>`. Optional for quotes and order tracking. Needed by `create_order` and `register_tx` when the gateway enforces a captcha. Request one from gateway@gobob.xyz. |
| `BOB_GATEWAY_MCP_URL` | `https://mcp.gobob.xyz/mcp` | Point the plugin at another Gateway MCP server. |

`create_order` creates a real order, and `register_tx` broadcasts a Bitcoin
transaction that cannot be undone. Leave both on ask-to-confirm.

## Other agents

Copy the skill directories into your agent's skills directory, and connect the MCP
server at `https://mcp.gobob.xyz/mcp` (streamable HTTP), with the optional
`Authorization: Bearer <key>` header. In Claude Code without the plugin:

```bash
claude mcp add --transport http bob-gateway https://mcp.gobob.xyz/mcp \
  --header "Authorization: Bearer $GATEWAY_API_KEY"
```

## Contributing

Each skill is a directory with a `SKILL.md`, whose frontmatter has a `name` equal to
the directory name and a `description` that starts with "Use when" and says when to
load the skill. A new skill joins the plugin once it is listed under `skills` in
[`.claude-plugin/marketplace.json`](../.claude-plugin/marketplace.json); bump the
plugin's `version` there with every change to a skill.

The skills are read by agents outside BOB: keep them to public endpoints, tools and
docs.
