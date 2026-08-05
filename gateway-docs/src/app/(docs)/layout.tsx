import { source } from '@/lib/source';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { baseOptions } from '@/lib/layout.shared';
import { SidebarLinks } from '@/components/sidebar-links';

export default function Layout({ children }: LayoutProps<'/'>) {
  return (
    <DocsLayout
      tree={source.getPageTree()}
      sidebar={{ footer: <SidebarLinks /> }}
      {...baseOptions()}
    >
      {children}
    </DocsLayout>
  );
}
