'use client';

import { useEffect, useId, useState } from 'react';
import { useTheme } from 'fumadocs-ui/provider/base';

/**
 * Renders the diagrams that remarkMdxMermaid extracts from ```mermaid fences.
 *
 * mermaid is imported lazily -- it is a few megabytes, and only one page uses
 * it, so it should not sit in the shared bundle.
 */
export function Mermaid({ chart }: { chart: string }) {
  const [svg, setSvg] = useState<string>();
  const { resolvedTheme } = useTheme();

  // mermaid needs a DOM-safe id, and useId() returns something like ":r0:".
  const id = `mermaid-${useId().replace(/[^a-zA-Z0-9]/g, '')}`;

  useEffect(() => {
    let active = true;

    void (async () => {
      const { default: mermaid } = await import('mermaid');

      mermaid.initialize({
        startOnLoad: false,
        securityLevel: 'strict',
        fontFamily: 'inherit',
        theme: resolvedTheme === 'dark' ? 'dark' : 'default',
      });

      try {
        const { svg } = await mermaid.render(id, chart.replaceAll('\\n', '\n'));
        if (active) setSvg(svg);
      } catch {
        // Leave the diagram blank rather than taking the page down on a syntax
        // error; the source stays visible in the repo either way.
        if (active) setSvg(undefined);
      }
    })();

    return () => {
      active = false;
    };
  }, [chart, id, resolvedTheme]);

  return (
    <div
      // Diagrams are wide; let them scroll rather than overflow the page.
      className="my-6 overflow-x-auto [&>svg]:mx-auto [&>svg]:h-auto [&>svg]:max-w-full"
      dangerouslySetInnerHTML={svg ? { __html: svg } : undefined}
    />
  );
}
