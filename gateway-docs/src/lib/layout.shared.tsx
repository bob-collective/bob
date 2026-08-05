import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { ChartLine, MessageCircle } from 'lucide-react';
import { appName, links } from './shared';

/**
 * Navigation carried over from the Mintlify docs.json: the two navbar links,
 * the "Launch App" primary button, and the global anchors.
 */
export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: (
        <>
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src="/gateway/dark.svg"
            alt={appName}
            width={60}
            height={28}
            className="block dark:hidden"
          />
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src="/gateway/light.svg"
            alt={appName}
            width={60}
            height={28}
            className="hidden dark:block"
          />
        </>
      ),
      url: '/gateway/overview',
    },
    // Homepage / BOB Docs / Launch App are rendered by <SidebarLinks /> in the
    // sidebar footer instead -- as nav links they land above the page tree.
    links: [
      {
        type: 'icon',
        label: 'Discord',
        text: 'Discord',
        icon: <MessageCircle />,
        url: links.discord,
        external: true,
      },
      {
        type: 'icon',
        label: 'Analytics',
        text: 'Analytics',
        icon: <ChartLine />,
        url: links.analytics,
        external: true,
      },
    ],
    githubUrl: links.github,
  };
}
