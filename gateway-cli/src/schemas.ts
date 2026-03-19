import { z } from "zod/v4";

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

const atomicAmount = z.string().regex(/^\d+$/, "must be a positive integer (atomic units)");

// ─── Amount: mutually exclusive group ───────────────────────────────────────

const amountGroup = z.object({
  amount: z.string().optional(),
  amountAtomic: atomicAmount.optional(),
  amountUsd: z.string().optional(),
}).check(
  (ctx) => {
    const provided = [ctx.value.amount, ctx.value.amountAtomic, ctx.value.amountUsd].filter(Boolean);
    if (provided.length === 0) {
      ctx.issues.push({ code: "custom", message: "one of --amount, --amount-atomic, or --amount-usd is required", input: ctx.value, path: [] });
    }
    if (provided.length > 1) {
      ctx.issues.push({ code: "custom", message: "--amount, --amount-atomic, and --amount-usd are mutually exclusive", input: ctx.value, path: [] });
    }
  },
);

// ─── Command schemas ────────────────────────────────────────────────────────

export const quoteSchema = z.object({
  src: z.string(),
  dst: z.string(),
  recipient: z.string().optional(),
  sender: z.string().optional(),
  slippage: positiveInt.optional(),
  gasRefillUsd: positiveNumber.optional(),
  btcFeeRate: positiveInt.optional(),
  json: z.boolean().default(false),
}).and(amountGroup);

export const swapSchema = quoteSchema.and(z.object({
  privateKey: z.string().optional(),
  wait: z.boolean().default(true),
  unsigned: z.boolean().default(false),
  timeout: positiveInt.optional(),
  retry: z.boolean().default(true),
}));

export type QuoteInput = z.input<typeof quoteSchema>;
export type SwapInput = z.input<typeof swapSchema>;
