import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { PublicKey } from "@solana/web3.js";
import { decodePortfolio, portfolioPda, PROGRAM_ID, recordExecutionIx } from "./program";

const vectors = JSON.parse(
  readFileSync(new URL("../../docs/golden-vectors.json", import.meta.url), "utf8"),
) as { state_hex: string; record_execution_hex: string };

describe("ABI", () => {
  it("derives the stable portfolio PDA", () => {
    const owner = new PublicKey("11111111111111111111111111111111");
    const [pda] = portfolioPda(owner);
    expect(PublicKey.isOnCurve(pda.toBytes())).toBe(false);
    expect(PROGRAM_ID.toBase58()).toBe("6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed");
  });

  it("decodes Rust state golden values as lossless bigint", () => {
    const state = decodePortfolio(Buffer.from(vectors.state_hex, "hex"));
    expect(state.name).toBe("Golden Portfolio");
    expect(state.status).toBe("PAUSED");
    expect(state.weights).toEqual([6000, 4000]);
    expect(state.version).toBe(9_007_199_254_740_993n);
    expect(state.lastExecutionId).toBe(9_007_199_254_740_995n);
    expect(state.trades).toBe(9_007_199_254_740_997n);
    expect(state.dailyTurnover).toBe(9_007_199_254_740_999n);
    expect(state.fees).toBe(9_007_199_254_741_001n);
    expect(state.turnoverDay).toBe(20_706n);
    expect(state.dailyTrades).toBe(7);
  });

  it("encodes RecordExecution exactly like the Rust ABI golden vector", () => {
    const owner = PublicKey.default;
    const ix = recordExecutionIx(owner, owner, {
      executionId: 42n,
      inputValue: 1_000_000n,
      outputValue: 996_000n,
      feeValue: 1_000n,
      side: 1,
      assetIndex: 1,
      portfolioNav: 10_000_000n,
    });
    expect(ix.data.toString("hex")).toBe(vectors.record_execution_hex);
  });
});
