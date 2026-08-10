import defaultMdxComponents from 'fumadocs-ui/mdx';
import { Callout } from 'fumadocs-ui/components/callout';
import { Card, Cards } from 'fumadocs-ui/components/card';
import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';
import { Step, Steps } from 'fumadocs-ui/components/steps';
import { Tab, Tabs } from 'fumadocs-ui/components/tabs';
import { resolveIcon } from './icons';
import { SupportedRoutes } from './supported-routes';
import { Mermaid } from './mermaid';
import type { MDXComponents } from 'mdx/types';
import type { ComponentProps } from 'react';

/** Accepts Mintlify's Font Awesome icon names as well as a plain node. */
function CardWithIcon({ icon, ...props }: ComponentProps<typeof Card>) {
  return <Card icon={resolveIcon(icon)} {...props} />;
}

/** Mintlify's <Accordion> took an icon; fumadocs' does not. */
function AccordionCompat({
  icon: _icon,
  ...props
}: ComponentProps<typeof Accordion> & { icon?: unknown }) {
  return <Accordion {...props} />;
}

export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultMdxComponents,
    Callout,
    Cards,
    Card: CardWithIcon,
    Accordions,
    Accordion: AccordionCompat,
    Steps,
    Step,
    Tabs,
    Tab,
    SupportedRoutes,
    Mermaid,
    ...components,
  } satisfies MDXComponents;
}

export const useMDXComponents = getMDXComponents;

declare global {
  type MDXProvidedComponents = ReturnType<typeof getMDXComponents>;
}
