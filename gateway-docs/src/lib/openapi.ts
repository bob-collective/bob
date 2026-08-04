import { createOpenAPI } from 'fumadocs-openapi/server';

/**
 * Shared OpenAPI server.
 *
 * The input path must stay identical to the one in
 * scripts/generate-openapi-pages.mjs -- fumadocs uses that string as the schema
 * ID and bakes it into every generated page, so the two have to agree.
 */
export const openapi = createOpenAPI({
  input: ['./openapi.json'],
});
