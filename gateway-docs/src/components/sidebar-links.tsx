import { ExternalLink } from 'lucide-react';
import { links } from '@/lib/shared';

const items = [
  { text: 'Homepage', url: links.homepage },
  { text: 'BOB Docs', url: links.bobDocs },
  { text: 'Launch App', url: links.launchApp },
];

/**
 * Rendered via the docs layout's `sidebar.footer` slot so these sit under the
 * page tree. Passing them as nav `links` instead puts them above it, which
 * buries the actual navigation.
 *
 * `order-first` lifts them above the icon/theme-switch bar, which fumadocs
 * renders as the first child of the same flex-col footer wrapper.
 */
export function SidebarLinks() {
  return (
    <div className="order-first mb-2 flex flex-col">
      {items.map((item) => (
        <a
          key={item.url}
          href={item.url}
          target="_blank"
          rel="noreferrer"
          className="flex items-center gap-2 rounded-lg px-2 py-1.5 text-sm text-fd-muted-foreground transition-colors hover:bg-fd-accent hover:text-fd-accent-foreground"
        >
          <ExternalLink className="size-4 shrink-0" />
          {item.text}
        </a>
      ))}
    </div>
  );
}
