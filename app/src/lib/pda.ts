import { PublicKey } from "@solana/web3.js";

export const getVaultPDA = async (eventId: string, programId: PublicKey) => {
  const [vaultPda] = await PublicKey.findProgramAddress(
    [Buffer.from("vault"), Buffer.from(eventId)],
    programId
  );
  return vaultPda;
};

export const getMetadataPDA = async (mint: PublicKey) => {
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const [metadataPda] = await PublicKey.findProgramAddress(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  );
  return metadataPda;
};
