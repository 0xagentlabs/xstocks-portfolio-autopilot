import {describe,expect,it} from "vitest"; import {PublicKey} from "@solana/web3.js"; import {portfolioPda,PROGRAM_ID} from "./program";
describe("ABI",()=>{it("derives stable portfolio PDA",()=>{const owner=new PublicKey("11111111111111111111111111111111");const [pda]=portfolioPda(owner);expect(PublicKey.isOnCurve(pda.toBytes())).toBe(false);expect(PROGRAM_ID.toBase58()).toBe("6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed")})});

