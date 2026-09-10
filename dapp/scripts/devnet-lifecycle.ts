import { Connection, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import {
  decodePortfolio,
  initializeIx,
  portfolioPda,
  recordExecutionIx,
  setStatusIx,
  updateStrategyIx,
} from "../lib/program";
import { DEFAULT_CONFIG, loadKeypair } from "./init";

async function main() {
  const keypairPath = process.argv.at(-1);
  if (!keypairPath || keypairPath === process.argv[1]) throw new Error("Usage: pnpm lifecycle:devnet -- <SOLANA_KEYPAIR_PATH>");
  const payer = loadKeypair(keypairPath);
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  const [state, bump] = portfolioPda(payer.publicKey);
  const signatures: Record<string, string> = {};
  const send = async (label: string, transaction: Transaction) => {
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    signatures[label] = signature;
  };

  let info = await connection.getAccountInfo(state);
  if (!info) {
    await send("initialize", new Transaction().add(initializeIx(payer.publicKey, state, bump, DEFAULT_CONFIG)));
    info = await connection.getAccountInfo(state);
  }
  if (!info) throw new Error("Portfolio account was not initialized");
  let portfolio = decodePortfolio(Buffer.from(info.data));
  if (portfolio.status !== "DRAFT") throw new Error(`Lifecycle requires DRAFT, found ${portfolio.status}`);

  const lifecycleConfig = { ...DEFAULT_CONFIG, maxDailyTrades: 2 };
  await send("update_strategy", new Transaction().add(updateStrategyIx(payer.publicKey, state, lifecycleConfig)));
  await send("running", new Transaction().add(setStatusIx(payer.publicKey, state, 1)));
  portfolio = decodePortfolio(Buffer.from((await connection.getAccountInfo(state))!.data));
  const firstId = portfolio.lastExecutionId + 1n;
  for (let index = 0; index < 2; index += 1) {
    await send(`record_${index + 1}`, new Transaction().add(recordExecutionIx(payer.publicKey, state, {
      executionId: firstId + BigInt(index), inputValue: 500_000n, outputValue: 498_000n,
      feeValue: 500n, side: index === 0 ? 0 : 1, assetIndex: index, portfolioNav: 10_000_000n,
    })));
  }

  const rejected = new Transaction().add(recordExecutionIx(payer.publicKey, state, {
    executionId: firstId + 2n, inputValue: 1n, outputValue: 1n, feeValue: 0n,
    side: 0, assetIndex: 0, portfolioNav: 10_000_000n,
  }));
  rejected.feePayer = payer.publicKey;
  rejected.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
  rejected.sign(payer);
  const simulation = await connection.simulateTransaction(rejected);
  if (!simulation.value.err) throw new Error("Expected max_daily_trades rejection");

  await send("paused", new Transaction().add(setStatusIx(payer.publicKey, state, 2)));
  await send("stopped", new Transaction().add(setStatusIx(payer.publicKey, state, 5)));
  const final = decodePortfolio(Buffer.from((await connection.getAccountInfo(state))!.data));
  console.log(JSON.stringify({ portfolio: state.toBase58(), signatures, rejected: simulation.value.err,
    final: { status: final.status, lastExecutionId: final.lastExecutionId.toString(),
      dailyTrades: final.dailyTrades, dailyTurnover: final.dailyTurnover.toString() } }, null, 2));
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
