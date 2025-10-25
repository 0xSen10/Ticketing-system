// app/src/lib/utils.ts
import { PublicKey } from "@solana/web3.js";

export function shortAddress(addr: string | PublicKey) {
  const str = typeof addr === "string" ? addr : addr.toBase58();
  return str.slice(0, 4) + "..." + str.slice(-4);
}

export function solToLamports(sol: number): number {
  return Math.floor(sol * 1_000_000_000);
}

export function lamportsToSol(lamports: number): number {
  return lamports / 1_000_000_000;
}
