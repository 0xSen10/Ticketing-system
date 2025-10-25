import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program, Idl } from "@project-serum/anchor";
import { useAnchorWallet } from "@solana/wallet-adapter-react";
import type { Wallet } from "@project-serum/anchor";

type AnchorWallet = Wallet & {
  publicKey: PublicKey;
  signTransaction: (tx: any) => Promise<any>;
  signAllTransactions: (txs: any[]) => Promise<any[]>;
};

const mockIdl: Idl = {
  version: "0.1.0",
  name: "ticket_platform",
  instructions: [],
  metadata: {
    address: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS", // 临时 placeholder，可替换为真实程序ID
  },
};

const PROGRAM_ID = new PublicKey(mockIdl.metadata!.address);

export function useTicketProgram() {
  const wallet = useAnchorWallet();
  const connection = new Connection("https://api.devnet.solana.com");
  const provider = new AnchorProvider(connection, wallet as AnchorWallet, { preflightCommitment: "processed" });
  
  const program = new Program(mockIdl as Idl, PROGRAM_ID, provider);
  return { program, provider };
}
