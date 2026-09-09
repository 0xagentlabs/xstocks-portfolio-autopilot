import { PublicKey, TransactionInstruction } from "@solana/web3.js";
export const PROGRAM_ID = new PublicKey("6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed");
export const STATUS = ["DRAFT","RUNNING","PAUSED","RISK_PAUSED","ERROR","STOPPED"] as const;
export function portfolioPda(owner: PublicKey){ return PublicKey.findProgramAddressSync([Buffer.from("portfolio"),owner.toBuffer()],PROGRAM_ID); }
export function setStatusIx(owner:PublicKey,state:PublicKey,status:number){ return new TransactionInstruction({programId:PROGRAM_ID,keys:[{pubkey:owner,isSigner:true,isWritable:false},{pubkey:state,isSigner:false,isWritable:true}],data:Buffer.from([2,status])}); }
export function decodePortfolio(d:Buffer){ if(d.length!==160||d.subarray(0,8).toString()!=="XSTOCKS1") throw new Error("Invalid portfolio account"); const u64=(o:number)=>Number(d.readBigUInt64LE(o)); return {owner:new PublicKey(d.subarray(8,40)),bump:d[40],status:STATUS[d[41]],assetCount:d[42],name:d.subarray(43,67).toString().replace(/\0+$/, ""),weights:Array.from({length:d[42]},(_,i)=>d.readUInt16LE(67+i*2)),core:Array.from({length:d[42]},(_,i)=>d.readUInt16LE(83+i*2)),rebalanceBps:d.readUInt16LE(99),slippageBps:d.readUInt16LE(101),maxDailyTrades:d.readUInt16LE(103),maxTurnoverBps:d.readUInt16LE(105),version:u64(107),lastExecutionId:u64(115),trades:u64(123),turnover:u64(131),fees:u64(139)}; }

