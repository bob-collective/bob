import { Command } from "commander";
import { ZodError } from "zod/v4";
import { isAddress } from "viem";
import { type OutputMode, createLogger, render, formatJson, formatConfirmation, formatChains, formatTokens, formatRoutes, formatBalance } from "./output.js";
import { quoteSchema, swapSchema } from "./schemas.js";
import { SwapError } from "./errors.js";
import { version } from '../package.json';

function modeOf(opts: { json?: boolean }): OutputMode {
  return opts.json ? "json" : "human";
}

const errorMessage = (err: unknown): string => {
  if (err instanceof ZodError) {
    return err.issues.map(i => `Invalid --${i.path.join(".")}: ${i.message}`).join("\n");
  }
  return err instanceof Error ? err.message : String(err);
};

function withErrorHandling(fn: (...args: any[]) => Promise<void>) {
  return async (...args: any[]) => {
    try {
      await fn(...args);
    } catch (err) {
      const mode = modeOf(args.find(a => a?.json !== undefined) ?? {});
      const msg = errorMessage(err);
      if (mode === "json") {
        const errJson: any = { error: { message: msg } };
        if (err instanceof SwapError) {
          for (const [k, v] of Object.entries(err.context)) {
            if (v !== undefined) errJson.error[k] = v;
          }
        }
        console.log(JSON.stringify(errJson, null, 2));
      } else {
        console.error(msg);
        if (err instanceof SwapError) {
          const c = err.context;
          if (c.orderId) console.error(`Order:    ${c.orderId}`);
          if (c.txParams) console.error(`Contract: ${c.txParams.to} (${c.txParams.chainName})`);
          if (c.srcAsset && c.dstAsset) console.error(`Route:    ${c.srcAsset.symbol}:${c.srcAsset.chain} → ${c.dstAsset.symbol}:${c.dstAsset.chain}`);
          if (c.functionSelector) console.error(`Selector: ${c.functionSelector}`);
          if (c.revertData) console.error(`Revert:   ${c.revertData}`);
        }
      }
      process.exitCode = 1;
    }
  };
}


export const program = new Command()
  .name("gateway-cli")
  .description("Swap between BTC and tokens like USDC, ETH, wBTC via BOB Gateway")
  .version(version);

program
  .command("routes")
  .description("List swap routes, supported chains, or tokens")
  .option("--src-chain <chain>", "Filter by source chain")
  .option("--dst-chain <chain>", "Filter by destination chain")
  .option("--chains", "List supported chains only", false)
  .option("--tokens <chain>", "List tokens available on a chain")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    const mode = modeOf(opts);
    const { handleRoutes } = await import("./commands/routes.js");
    const result = await handleRoutes({ from: opts.srcChain, to: opts.dstChain, chains: opts.chains, tokens: opts.tokens });
    if (result.type === "chains") render(result.data, mode, formatChains);
    else if (result.type === "tokens") render(result.data, mode, formatTokens);
    else render(result.data, mode, formatRoutes);
  }));

program
  .command("quote")
  .description("Get a swap quote")
  .requiredOption("--src <asset[:chain]>", "Source asset (e.g. BTC, USDC:ethereum)")
  .requiredOption("--dst <asset[:chain]>", "Destination asset (e.g. USDC:ethereum, BTC)")
  .requiredOption("--amount <value>", "Amount: 0.05BTC, 100USDC, 100USD, 5000000 (atomic), ALL")
  .option("--recipient <address>", "Recipient address")
  .option("--sender <address>", "Sender address")
  .option("--owner <address>", "Order owner address (EVM 0x... or Tron T...)")
  .option("--slippage <bps>", "Slippage in basis points")
  .option("--gas-refill-usd <usd>", "Request ETH gas refill on destination (USD amount)")
  .option("--btc-fee-rate <sat/vbyte>", "Bitcoin fee rate (default: mempool.space next-block)")
  .option("--fee-token <address>", "ERC20 token used to pay gas (paymaster)")
  .option("--fee-reserve <amount>", "Amount of fee token to reserve for gas, in atomic units (e.g. wei)")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    const mode = modeOf(opts);
    const parsed = quoteSchema.parse(opts);
    const { handleQuote } = await import("./commands/quote.js");
    const result = await handleQuote({ ...parsed, sender: parsed.sender, ownerAddress: parsed.owner });
    render(result.quote, mode, () => formatConfirmation(result.confirmation));
  }));

program
  .command("swap")
  .description("Execute a cross-chain swap (e.g. BTC to USDC, ETH to BTC)")
  .requiredOption("--src <asset[:chain]>", "Source asset (e.g. BTC, USDC:ethereum)")
  .requiredOption("--dst <asset[:chain]>", "Destination asset (e.g. USDC:ethereum, BTC)")
  .requiredOption("--amount <value>", "Amount: 0.05BTC, 100USDC, 100USD, 5000000 (atomic), ALL")
  .option("--recipient <address>", "Recipient address")
  .option("--sender <address>", "Sender address")
  .option("--owner <address>", "Order owner address (EVM 0x... or Tron T...)")
  .option("--slippage <bps>", "Slippage in basis points")
  .option("--gas-refill-usd <usd>", "Request ETH gas refill on destination (USD amount)")
  .option("--btc-fee-rate <sat/vbyte>", "Bitcoin fee rate (default: mempool.space)")
  .option("--fee-token <address>", "ERC20 token used to pay gas (paymaster)")
  .option("--fee-reserve <amount>", "Amount of fee token to reserve for gas, in atomic units (e.g. wei)")
  .option("--private-key <key>", "Private key (WIF for BTC, hex for EVM/Tron). WARNING: visible in process listings — prefer env vars")
  .option("--no-wait", "Exit after submitting without polling")
  .option("--unsigned", "Output unsigned PSBT/tx data without signing", false)
  .option("--timeout <seconds>", "Polling timeout in seconds", "1800")
  .option("--no-retry", "Fail immediately on transient errors")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    const mode = modeOf(opts);
    const parsed = swapSchema.parse(opts);

    const log = createLogger(mode);
    const { handleSwap } = await import("./commands/swap.js");
    const result = await handleSwap({ ...parsed, sender: parsed.sender, owner: parsed.owner }, log);

    render("data" in result ? result.data : result, mode);
  }));

program
  .command("status <order-id>")
  .description("Check order status")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (orderId, opts) => {
    const { handleStatus } = await import("./commands/status.js");
    render(await handleStatus({ orderId }), modeOf(opts));
  }));

program
  .command("orders <address>")
  .description("List all orders for an address")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (address, opts) => {
    const { handleOrders } = await import("./commands/orders.js");
    render(await handleOrders({ address }), modeOf(opts));
  }));

program
  .command("balance")
  .description("Show token balances on supported chains")
  .argument("[addresses...]", "Wallet addresses (BTC, EVM, or Tron). Omit to derive from env var keys.")
  .option("--chain <chain...>", "Chains to check (repeatable, comma-separated)")
  .option("--fee-token <address>", "ERC20 token used to pay gas (paymaster)")
  .option("--fee-reserve <amount>", "Amount of fee token to reserve for gas, in atomic units (e.g. wei)")
  .option("--non-zero", "Only show chains with non-zero balances", false)
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (addresses, opts) => {
    if (opts.feeToken != null && !isAddress(opts.feeToken, { strict: false })) {
      throw new Error("Invalid --fee-token: must be a valid EVM address");
    }
    if (opts.feeReserve != null) {
      if (!/^\d+$/.test(opts.feeReserve)) throw new Error("Invalid --fee-reserve: must be a non-negative integer (no scientific notation)");
    }
    const chains = opts.chain?.flatMap((c: string) => c.split(",").map((s: string) => s.trim())).filter(Boolean);
    const { handleBalance } = await import("./commands/balance.js");
    render(await handleBalance(addresses, { chain: chains, feeToken: opts.feeToken, feeReserve: opts.feeReserve, nonZero: opts.nonZero }), modeOf(opts), formatBalance);
  }));

program
  .command("register <order-id> <txid>")
  .description("Register a tx for an existing order (recovery)")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (orderId, txid, opts) => {
    const { handleRegister } = await import("./commands/register.js");
    render(await handleRegister({ orderId, txid }), modeOf(opts));
  }));
