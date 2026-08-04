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
            src="/dark.svg"
            alt={appName}
            width={60}
            height={28}
            className="block dark:hidden"
          />
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src="/light.svg"
            alt={appName}
            width={60}
            height={28}
            className="hidden dark:block"
          />
        </>
      ),
      url: '/gateway/overview',
    },
    links: [
      { text: 'Homepage', url: links.homepage, external: true },
      { text: 'BOB Docs', url: links.bobDocs, external: true },
      { type: 'button', text: 'Launch App', url: links.launchApp, external: true },
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
