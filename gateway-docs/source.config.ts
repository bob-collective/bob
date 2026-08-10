import { defineConfig } from 'fumadocs-mdx/config';
import { remarkMdxMermaid } from 'fumadocs-core/mdx-plugins/remark-mdx-mermaid';

/**
 * Global MDX options. Collections are declared with the macro API in
 * src/lib/source.ts; this only carries settings that have to merge with
 * fumadocs' own plugin preset rather than replace it.
 */
export default defineConfig({
  mdxOptions: {
    // Turns ```mermaid fences into <Mermaid />, which src/components/mdx.tsx
    // maps to the renderer. Mintlify drew these natively; fumadocs does not.
    // Passed uninvoked: unified calls the plugin factory itself at freeze time.
    remarkPlugins: (plugins) => [remarkMdxMermaid, ...plugins],
  },
});
