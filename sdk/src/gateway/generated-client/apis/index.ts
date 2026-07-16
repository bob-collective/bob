/* tslint:disable */
/* eslint-disable */
export * from './V1Api';
export * from './V2Api';
export * from './V3Api';
// STOPGAP: the spec already gives v3 its own operationId
// (bob-gateway#1658), but this client was generated before that landed, so both V2Api and
// V3Api still declare `GetMaxSpendableV2Request` — an ambiguous re-export (TS2308) that
// breaks `tsc`. This explicit re-export wins over the star exports and picks V2's copy (the
// two are structurally identical). Delete this line the next time `pnpm codegen` regenerates
// the client from the current spec.
export type { GetMaxSpendableV2Request } from './V2Api';
