import { z } from "zod/v4";
import { isAddress } from "viem";

// ─── Reusable field schemas ─────────────────────────────────────────────────

/**
 * Schema for positive decimal numbers (e.g., gas refill USD amount).
 * Transforms string input to number, validates > 0.
 */
const positiveNumber = z.string().transform((v, ctx) => {
  const n = parseFloat(v);
  if (isNaN(n) || n <= 0) {
    ctx.addIssue({ code: "custom", message: `invalid number "${v}" — must be a positive number`, input: v });
    return z.NEVER;
  }
  return n;
});

/**
 * Schema for positive integers (e.g., slippage BPS, timeout seconds).
 * Transforms string input to number, validates > 0 and whole number.
 */
const positiveInt = z.string().transform((v, ctx) => {
  const n = Number(v);
  if (isNaN(n) || !Number.isInteger(n) || n <= 0) {
    ctx.addIssue({ code: "custom", message: `invalid integer "${v}" — must be a positive whole number`, input: v });
    return z.NEVER;
  }
  return n;
});

// ─── Command schemas ────────────────────────────────────────────────────────

/**
 * Zod schema for quote command input validation.
 * Validates all required and optional fields for getting a swap quote.
 */
export const quoteSchema = z.object({
  src: z.string(),
  dst: z.string(),
  amount: z.string(),
  recipient: z.string().optional(),
  sender: z.string().optional(),
  slippage: positiveInt.optional(),
  gasRefillUsd: positiveNumber.optional(),
  btcFeeRate: positiveInt.optional(),
  feeToken: z.string().refine(v => isAddress(v, { strict: false }), { message: "must be a valid EVM address" }).optional(),
  feeReserve: z.string().refine(v => /^\d+$/.test(v), { message: "must be a non-negative integer (no scientific notation)" }).optional(),
  json: z.boolean().default(false),
});

/**
 * Zod schema for swap command input validation.
 * Extends quoteSchema with additional swap-specific fields.
 */
export const swapSchema = quoteSchema.and(z.object({
  privateKey: z.string().optional(),
  wait: z.boolean().default(true),
  unsigned: z.boolean().default(false),
  timeout: positiveInt.pipe(z.number().min(1, "timeout must be >= 1")).optional(),
  retry: z.boolean().default(true),
}));

/** TypeScript type inferred from quoteSchema input. */
export type QuoteInput = z.input<typeof quoteSchema>;
/** TypeScript type inferred from swapSchema input. */
export type SwapInput = z.input<typeof swapSchema>;
