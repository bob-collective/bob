'use client';

import { createOpenAPIPage } from 'fumadocs-openapi/ui';

/**
 * The interactive reference block rendered by the generated API pages. Kept in
 * its own module so the client bundle is only pulled in by pages that use it.
 */
export const OpenAPIPage = createOpenAPIPage();
