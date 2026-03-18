import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import type { RouteInfo } from '@gobob/bob-sdk';

const CACHE_DIR = path.join(os.homedir(), '.gateway-cli', 'cache');
const CACHE_FILE = path.join(CACHE_DIR, 'routes.json');
const DEFAULT_TTL = '24h';

interface CacheEntry {
  fetchedAt: string;
  routes: RouteInfo[];
}

export function parseTtl(ttl: string): number {
  const match = ttl.match(/^(\d+)(h|d)$/);
  if (!match) return 24 * 60 * 60 * 1000;
  const [, num, unit] = match;
  return unit === 'h' ? Number(num) * 3600_000 : Number(num) * 86400_000;
}

export function isCacheValid(ttl: string = DEFAULT_TTL): boolean {
  if (!fs.existsSync(CACHE_FILE)) return false;
  try {
    const data: CacheEntry = JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'));
    const age = Date.now() - new Date(data.fetchedAt).getTime();
    return age < parseTtl(ttl);
  } catch { return false; }
}

export function readCache(): RouteInfo[] | null {
  if (!fs.existsSync(CACHE_FILE)) return null;
  try {
    const data: CacheEntry = JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'));
    return data.routes;
  } catch { return null; }
}

export function writeCache(routes: RouteInfo[]): void {
  fs.mkdirSync(CACHE_DIR, { recursive: true });
  const entry: CacheEntry = { fetchedAt: new Date().toISOString(), routes };
  fs.writeFileSync(CACHE_FILE, JSON.stringify(entry));
}

export async function getOrFetchRoutes(
  fetchRoutes: () => Promise<RouteInfo[]>,
  ttl: string = DEFAULT_TTL,
  noCache: boolean = false,
): Promise<RouteInfo[]> {
  if (!noCache && isCacheValid(ttl)) {
    const cached = readCache();
    if (cached) return cached;
  }
  const routes = await fetchRoutes();
  writeCache(routes);
  return routes;
}
