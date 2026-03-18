import { Command } from "commander";
import { loadConfig } from "./config/index.js";
import { handleChains } from "./commands/chains.js";
import { handleTokens } from "./commands/tokens.js";
import { handleQuote } from "./commands/quote.js";
import { handleStatus } from "./commands/status.js";
import { handleOrders } from "./commands/orders.js";
import { handleMaxSpendable } from "./commands/max-spendable.js";
import { handleSwap, RegistrationError } from "./commands/swap.js";
import { handleRegister } from "./commands/register.js";
import { handleBalance } from "./commands/balance.js";
import { PollTimeoutError } from "./polling/poll-order.js";
import { PriceOracleError } from "./util/price-oracle.js";
import { TransientError } from "./util/retry.js";
import { setJsonMode, isJsonMode, setVerboseMode } from "./util/progress.js";

const errorMessage = (err: unknown): string => err instanceof Error ? err.message : String(err);

function withErrorHandling(fn: (...args: any[]) => Promise<void>) {
  return async (...args: any[]) => {
    try {
      await fn(...args);
    } catch (err) {
      const msg = errorMessage(err);
      if (isJsonMode()) {
        console.log(JSON.stringify({ error: { code: 1, message: msg } }, null, 2));
      } else {
        console.error(msg);
      }
      process.exitCode = 1;
    }
  };
}

function withTxErrorHandling(fn: (...args: any[]) => Promise<void>) {
  return async (...args: any[]) => {
    try {
      await fn(...args);
    } catch (err) {
      const msg = errorMessage(err);

      if (err instanceof TransientError) {
        if (isJsonMode()) {
          console.log(JSON.stringify({ error: { code: 6, message: msg, retryable: true } }, null, 2));
        } else {
          console.error(msg);
        }
        process.exit(6);
      }

      let code = 1;
      if (err instanceof RegistrationError) code = 3;
      else if (err instanceof PollTimeoutError) code = 2;
      else if (err instanceof PriceOracleError) code = 4;
      else if (err instanceof Error && /insufficient funds/i.test(err.message)) code = 5;

      if (isJsonMode()) {
        const errJson: any = { error: { code, message: msg } };
        if (err instanceof RegistrationError) errJson.error.orderId = err.orderId;
        console.log(JSON.stringify(errJson, null, 2));
      } else {
        console.error(msg);
      }
      process.exit(code);
    }
  };
}

export const program = new Command()
  .name("gateway-cli")
  .description("Bridge Bitcoin to any chain via BOB Gateway")
  .version("0.2.0")
  .option("-v, --verbose", "Enable verbose logging to stderr", false)
  .hook("preAction", (thisCommand) => {
    const opts = thisCommand.opts();
    if (opts.verbose) setVerboseMode(true);
  });

program
  .command("chains")
  .description("List all supported chains with aliases and chain IDs")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleChains({ json: opts.json }));
  }));

program
  .command("tokens")
  .description("List available tokens on a chain")
  .requiredOption("--chain <chain>", "Chain name or alias")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleTokens({ chain: opts.chain, json: opts.json }));
  }));

program
  .command("routes")
  .description("List available bridge routes")
  .option("--src-chain <chain>", "Filter by source chain")
  .option("--dst-chain <chain>", "Filter by destination chain")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    if (opts.json) setJsonMode(true);
    const { handleRoutes } = await import("./commands/routes.js");
    console.log(await handleRoutes({ json: opts.json, from: opts.srcChain, to: opts.dstChain }));
  }));

program
  .command("quote")
  .description("Get a bridge quote")
  .requiredOption("--src <asset[:chain]>", "Source asset (e.g. BTC, USDC:ethereum)")
  .requiredOption("--dst <asset[:chain]>", "Destination asset (e.g. USDC:ethereum, BTC)")
  .requiredOption("--amount <amt>", "Amount with suffix (e.g. 0.1BTC, 100USDC, $50)")
  .option("--recipient <address>", "Recipient address (env: GATEWAY_RECIPIENT)", process.env.GATEWAY_RECIPIENT)
  .option("--sender <address>", "Sender address (env: GATEWAY_SENDER)", process.env.GATEWAY_SENDER)
  .option("--slippage <bps>", "Slippage in basis points")
  .option("--gas-refill <usd>", "Request ETH gas refill on destination (USD amount)")
  .option("--btc-fee-rate <sat/vbyte>", "Bitcoin fee rate (default: mempool.space next-block)")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (opts) => {
    const config = loadConfig();
    if (opts.json) setJsonMode(true);
    const slippage = opts.slippage ? parseInt(opts.slippage, 10) : config.slippageBps;
    console.log(await handleQuote({
      src: opts.src,
      dst: opts.dst,
      amount: opts.amount,
      recipient: opts.recipient ?? config.recipient,
      sender: opts.sender ?? config.sender,
      slippageBps: slippage,
      gasRefillUsd: opts.gasRefill ? parseFloat(opts.gasRefill) : undefined,
      btcFeeRate: opts.btcFeeRate ? parseInt(opts.btcFeeRate, 10) : config.btcFeeRate,
      json: opts.json,
    }));
  }));

function addSwapOptions(cmd: Command): Command {
  return cmd
    .requiredOption("--src <asset[:chain]>", "Source asset (e.g. BTC, USDC:ethereum)")
    .requiredOption("--dst <asset[:chain]>", "Destination asset (e.g. USDC:ethereum, BTC)")
    .requiredOption("--amount <amt>", "Amount with suffix (e.g. 0.1BTC, 100USDC, $50)")
    .option("--recipient <address>", "Recipient address (env: GATEWAY_RECIPIENT)", process.env.GATEWAY_RECIPIENT)
    .option("--sender <address>", "Sender address (env: GATEWAY_SENDER)", process.env.GATEWAY_SENDER)
    .option("--slippage <bps>", "Slippage in basis points")
    .option("--gas-refill <usd>", "Request ETH gas refill on destination (USD amount)")
    .option("--btc-fee-rate <sat/vbyte>", "Bitcoin fee rate (default: mempool.space)")
    .option("--private-key <key>", "Private key (WIF for BTC, hex for EVM)")
    .option("--keystore <path>", "Path to EVM JSON keystore file (env: GATEWAY_KEYSTORE)", process.env.GATEWAY_KEYSTORE)
    .option("--password <pass>", "Keystore password (or KEYSTORE_PASSWORD env)")
    .option("--auto-confirm", "Skip confirmation prompt", false)
    .option("--dry-run", "Show quote and exit without creating an order", false)
    .option("--no-wait", "Exit after submitting without polling")
    .option("--unsigned", "Output unsigned PSBT/tx data without signing", false)
    .option("--timeout <seconds>", "Polling timeout (env: GATEWAY_TIMEOUT)", "1800")
    .option("--no-retry", "Fail immediately on transient errors (exit code 6)")
    .option("--json", "Output as JSON", false);
}

function buildSwapArgs(opts: any, config: ReturnType<typeof loadConfig>) {
  const slippage = opts.slippage ? parseInt(opts.slippage, 10) : config.slippageBps;
  const timeout = opts.timeout ? parseInt(opts.timeout, 10) * 1000 : config.timeoutMs;
  return {
    apiUrl: config.apiUrl,
    src: opts.src,
    dst: opts.dst,
    amount: opts.amount,
    recipient: opts.recipient ?? config.recipient,
    sender: opts.sender ?? config.sender,
    slippageBps: slippage,
    gasRefillUsd: opts.gasRefill ? parseFloat(opts.gasRefill) : undefined,
    btcFeeRate: opts.btcFeeRate ? parseInt(opts.btcFeeRate, 10) : config.btcFeeRate,
    privateKey: opts.privateKey,
    keystorePath: opts.keystore ?? config.keystorePath,
    keystorePassword: opts.password ?? config.keystorePassword,
    evmRpcUrl: config.evmRpcUrl,
    bitcoinPrivateKey: config.bitcoinPrivateKey,
    evmPrivateKey: config.evmPrivateKey,
    bitcoinSignerCmd: config.bitcoinSigner,
    evmSignerCmd: config.evmSigner,
    autoConfirm: opts.autoConfirm || config.autoConfirm,
    unsigned: opts.unsigned,
    dryRun: opts.dryRun,
    noWait: opts.wait === false || config.noWait,
    noRetry: opts.retry === false,
    timeoutMs: timeout,
    json: opts.json,
  };
}

addSwapOptions(program.command("swap").description("Bridge BTC to/from another chain"))
  .action(withTxErrorHandling(async (opts) => {
    const config = loadConfig();
    if (opts.json) setJsonMode(true);
    console.log(await handleSwap(buildSwapArgs(opts, config)));
  }));

// Hidden alias for backwards compatibility
{
  const offrampCmd = addSwapOptions(new Command("offramp").description("(alias for swap)"))
    .action(withTxErrorHandling(async (opts) => {
      const config = loadConfig();
      if (opts.json) setJsonMode(true);
      console.log(await handleSwap(buildSwapArgs(opts, config)));
    }));
  program.addCommand(offrampCmd, { hidden: true });
}

program
  .command("status <order-id>")
  .description("Check order status")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (orderId, opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleStatus({ orderId, json: opts.json }));
  }));

program
  .command("orders <address>")
  .description("List all orders for an address")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (address, opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleOrders({ address, json: opts.json }));
  }));

program
  .command("max-spendable <address>")
  .description("Check maximum spendable BTC after fees")
  .option("--btc-fee-rate <sat/vbyte>", "Bitcoin fee rate (default: mempool.space)")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (address, opts) => {
    if (opts.json) setJsonMode(true);
    const config = loadConfig();
    console.log(await handleMaxSpendable({
      address,
      btcFeeRate: opts.btcFeeRate ? parseInt(opts.btcFeeRate, 10) : config.btcFeeRate,
      json: opts.json,
    }));
  }));

program
  .command("balance")
  .description("Show token balances across gateway-supported chains")
  .argument("<address>", "Wallet address (BTC or EVM)")
  .option("--chain <chain>", "Specific chain to check")
  .option("--json", "Output as JSON", false)
  .option("--no-cache", "Skip route cache")
  .action(withErrorHandling(async (address, opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleBalance(address, {
      chain: opts.chain,
      json: opts.json,
      noCache: opts.cache === false,
    }));
  }));

program
  .command("register <order-id> <txid>")
  .description("Manually register a signed tx (recovery)")
  .option("--json", "Output as JSON", false)
  .action(withErrorHandling(async (orderId, txid, opts) => {
    if (opts.json) setJsonMode(true);
    console.log(await handleRegister({ orderId, txid, json: opts.json }));
  }));
