// src/util/progress.ts

let jsonMode = false;
let verboseMode = false;

export function setVerboseMode(enabled: boolean) {
  verboseMode = enabled;
}

export function verbose(msg: string) {
  if (verboseMode) process.stderr.write(`[verbose] ${msg}\n`);
}

export function setJsonMode(enabled: boolean) {
  jsonMode = enabled;
}

export function progress(msg: string) {
  if (!jsonMode) process.stderr.write(msg + "\n");
}

export function warn(msg: string) {
  if (!jsonMode) process.stderr.write(`Warning: ${msg}\n`);
}

export function isJsonMode(): boolean {
  return jsonMode;
}
