import { PublicKey, SYSVAR_CLOCK_PUBKEY, SystemProgram, TransactionInstruction } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey("6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed");
export const STATUS = ["DRAFT", "RUNNING", "PAUSED", "RISK_PAUSED", "ERROR", "STOPPED"] as const;

export type StrategyConfig = {
  assetCount: number;
  name: string;
  weights: readonly number[];
  coreRatios: readonly number[];
  rebalanceBps: number;
  slippageBps: number;
  maxDailyTrades: number;
  maxTurnoverBps: number;
};

export function portfolioPda(owner: PublicKey) {
  return PublicKey.findProgramAddressSync([Buffer.from("portfolio"), owner.toBuffer()], PROGRAM_ID);
}

export function encodeConfig(tag: 0 | 1, bump: number, config: StrategyConfig) {
  const data = Buffer.alloc(67);
  data[0] = tag;
  data[1] = tag === 0 ? bump : 0;
  data[2] = config.assetCount;
  Buffer.from(config.name, "utf8").copy(data, 3, 0, 24);
  config.weights.slice(0, 8).forEach((value, index) => data.writeUInt16LE(value, 27 + index * 2));
  config.coreRatios.slice(0, 8).forEach((value, index) => data.writeUInt16LE(value, 43 + index * 2));
  data.writeUInt16LE(config.rebalanceBps, 59);
  data.writeUInt16LE(config.slippageBps, 61);
  data.writeUInt16LE(config.maxDailyTrades, 63);
  data.writeUInt16LE(config.maxTurnoverBps, 65);
  return data;
}

export function initializeIx(owner: PublicKey, state: PublicKey, bump: number, config: StrategyConfig) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: true },
      { pubkey: state, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: encodeConfig(0, bump, config),
  });
}

export function updateStrategyIx(owner: PublicKey, state: PublicKey, config: StrategyConfig) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: false },
      { pubkey: state, isSigner: false, isWritable: true },
    ],
    data: encodeConfig(1, 0, config),
  });
}

export function setStatusIx(owner: PublicKey, state: PublicKey, status: number) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: false },
      { pubkey: state, isSigner: false, isWritable: true },
    ],
    data: Buffer.from([2, status]),
  });
}

export function recordExecutionIx(
  owner: PublicKey,
  state: PublicKey,
  execution: {
    executionId: bigint;
    inputValue: bigint;
    outputValue: bigint;
    feeValue: bigint;
    side: 0 | 1;
    assetIndex: number;
    portfolioNav: bigint;
  },
) {
  const data = Buffer.alloc(43);
  data[0] = 3;
  data.writeBigUInt64LE(execution.executionId, 1);
  data.writeBigUInt64LE(execution.inputValue, 9);
  data.writeBigUInt64LE(execution.outputValue, 17);
  data.writeBigUInt64LE(execution.feeValue, 25);
  data[33] = execution.side;
  data[34] = execution.assetIndex;
  data.writeBigUInt64LE(execution.portfolioNav, 35);
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: owner, isSigner: true, isWritable: false },
      { pubkey: state, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
    ],
    data,
  });
}

export function decodePortfolio(d: Buffer) {
  if (d.length !== 160 || d.subarray(0, 8).toString() !== "XSTOCKS1") throw new Error("Invalid portfolio account");
  const assetCount = d[42];
  if (assetCount === 0 || assetCount > 8 || d[41] >= STATUS.length) throw new Error("Invalid portfolio account fields");
  const u64 = (offset: number): bigint => d.readBigUInt64LE(offset);
  return {
    owner: new PublicKey(d.subarray(8, 40)), bump: d[40], status: STATUS[d[41]], assetCount,
    name: d.subarray(43, 67).toString().replace(/\0+$/, ""),
    weights: Array.from({ length: assetCount }, (_, i) => d.readUInt16LE(67 + i * 2)),
    core: Array.from({ length: assetCount }, (_, i) => d.readUInt16LE(83 + i * 2)),
    rebalanceBps: d.readUInt16LE(99), slippageBps: d.readUInt16LE(101),
    maxDailyTrades: d.readUInt16LE(103), maxTurnoverBps: d.readUInt16LE(105),
    version: u64(107), lastExecutionId: u64(115), trades: u64(123),
    dailyTurnover: u64(131), fees: u64(139), turnoverDay: d.readBigInt64LE(147),
    dailyTrades: d.readUInt16LE(155),
  };
}
