import { readFileSync } from "node:fs";
import { Connection, Keypair, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { PROGRAM_ID, portfolioPda } from "../lib/program";

async function main() {
const keypairPath = process.argv.at(-1);
if (!keypairPath || keypairPath === process.argv[1]) throw new Error("Usage: pnpm init:devnet -- <SOLANA_KEYPAIR_PATH>");
const payer = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(keypairPath, "utf8"))));
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const [pda, bump] = portfolioPda(payer.publicKey);
if (await connection.getAccountInfo(pda)) { console.log(`Portfolio already initialized: ${pda}`); process.exit(0); }
const data = Buffer.alloc(67); data[0]=0; data[1]=bump; data[2]=4; data.write("AI Growth",3,"utf8");
[4000,3000,2500,500].forEach((v,i)=>data.writeUInt16LE(v,27+i*2));
[8000,7000,9000,10000].forEach((v,i)=>data.writeUInt16LE(v,43+i*2));
data.writeUInt16LE(500,59); data.writeUInt16LE(50,61); data.writeUInt16LE(10,63); data.writeUInt16LE(2000,65);
const ix=new TransactionInstruction({programId:PROGRAM_ID,keys:[{pubkey:payer.publicKey,isSigner:true,isWritable:true},{pubkey:pda,isSigner:false,isWritable:true},{pubkey:SystemProgram.programId,isSigner:false,isWritable:false}],data});
const signature=await connection.sendTransaction(new Transaction().add(ix),[payer]); await connection.confirmTransaction(signature,"confirmed");
console.log(`Portfolio PDA: ${pda}`); console.log(`Signature: ${signature}`);
}
main().catch((error)=>{ console.error(error instanceof Error ? error.message : error); process.exit(1); });
