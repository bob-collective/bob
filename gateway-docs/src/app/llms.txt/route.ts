import { source } from '@/lib/source';
import { appName, siteUrl } from '@/lib/shared';
import { llms } from 'fumadocs-core/source';

export const revalidate = false;

// The public API serves the same spec as `openapi.json`.
const apiSpecUrl = 'https://gateway-api-mainnet.gobob.xyz/api-doc.json';

export function GET() {
  const { indexNode } = llms(source);
  const index = source.getPageTree().children.map((node) => indexNode(node));

  return new Response(
    [
      `# ${appName}`,
      '',
      '> Swap native BTC to and from tokens on other chains. Non-custodial: users sign every transaction with their own wallet.',
      '',
      'Every page below is also available as Markdown: append `.md` to its URL.',
      `- [All pages in one file](${siteUrl}/gateway/llms-full.txt)`,
      `- [OpenAPI spec](${apiSpecUrl})`,
      `- [Hosted MCP server](${siteUrl}/gateway/agents#connect-an-agent-over-mcp): \`https://mcp.gobob.xyz/mcp\` (streamable HTTP) quotes, creates and tracks orders; it never signs`,
      '',
      ...index,
    ].join('\n'),
  );
}
