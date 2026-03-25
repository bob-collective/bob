import { z } from "zod/v4";
import { isAddress } from "viem";

// ─── Reusable field schemas ─────────────────────────────────────────────────

const positiveNumber = z.string().transform((v, ctx) => {
  const n = parseFloat(v);
  if (isNaN(n) || n < 0) {
    ctx.addIssue({ code: "custom", message: `invalid number "${v}"`, input: v });
    return z.NEVER;
  }
  return n;
});

const positiveInt = z.string().transform((v, ctx) => {
  const n = parseInt(v, 10);
  if (isNaN(n) || n < 0) {
    ctx.addIssue({ code: "custom", message: `invalid integer "${v}"`, input: v });
    return z.NEVER;
  }
  return n;
});

// ─── Command schemas ────────────────────────────────────────────────────────

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
  feeReserve: z.string().refine(v => { const n = Number(v); return Number.isInteger(n) && n >= 0; }, { message: "must be a non-negative integer" }).optional(),
  json: z.boolean().default(false),
});

export const swapSchema = quoteSchema.and(z.object({
  privateKey: z.string().optional(),
  wait: z.boolean().default(true),
  unsigned: z.boolean().default(false),
  timeout: positiveInt.pipe(z.number().min(1, "timeout must be >= 1")).optional(),
  retry: z.boolean().default(true),
}));

export type QuoteInput = z.input<typeof quoteSchema>;
export type SwapInput = z.input<typeof swapSchema>;
