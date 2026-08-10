import * as lucide from 'lucide-react';
import faToLucide from '@/lib/fa-to-lucide.json';
import type { ReactNode } from 'react';

const map = faToLucide as Record<string, string | null>;

/**
 * Mintlify passed icons as Font Awesome name strings (icon="layer-group");
 * fumadocs expects a node. Resolve the name to a Lucide component, and render
 * nothing for names we deliberately dropped.
 *
 * Next's optimizePackageImports covers lucide-react, so the namespace import
 * does not pull the whole icon set into the bundle.
 */
export function resolveIcon(icon: ReactNode): ReactNode {
  if (typeof icon !== 'string') return icon;

  const name = map[icon];
  if (!name) return undefined;

  const Icon = (lucide as unknown as Record<string, lucide.LucideIcon | undefined>)[name];
  return Icon ? <Icon /> : undefined;
}
