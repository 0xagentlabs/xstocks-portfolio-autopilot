import { readFileSync } from "node:fs";
import { Connection, Keypair, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { initializeIx, portfolioPda, StrategyConfig } from "../lib/program";

export const DEFAULT_CONFIG: StrategyConfig = {
  assetCount: 4,
  name: "AI Growth",
  weights: [4000, 3000, 2500, 500],
  coreRatios: [8000, 7000, 9000, 10000],
  rebalanceBps: 500,
  slippageBps: 50,
  maxDailyTrades: 10,
  maxTurnoverBps: 2000,
};

export function loadKeypair(path: string) {
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(path, "utf8"))));
}

async function main() {
  const keypairPath = process.argv.at(-1);
  if (!keypairPath || keypairPath === process.argv[1]) throw new Error("Usage: pnpm init:devnet -- <SOLANA_KEYPAIR_PATH>");
  const payer = loadKeypair(keypairPath);
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  const [pda, bump] = portfolioPda(payer.publicKey);
  if (await connection.getAccountInfo(pda)) {
    console.log(`Portfolio already initialized: ${pda}`);
    return;
  }
  const signature = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(initializeIx(payer.publicKey, pda, bump, DEFAULT_CONFIG)),
    [payer],
  );
  console.log(`Portfolio PDA: ${pda}`);
  console.log(`Initialize: ${signature}`);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((error) => {
    console.error(error instanceof Error ? error.message : error);
    process.exit(1);
  });
}
