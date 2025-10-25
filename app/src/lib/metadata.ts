// app/src/lib/metadata.ts
/**
 * NFT Metadata 生成工具
 * 返回标准 Metaplex Metadata JSON
 */
export function generateMetadata(eventName: string, description: string, imageUrl: string) {
  return {
    name: eventName,
    symbol: "TICKET",
    description,
    image: imageUrl,
    attributes: [
      { trait_type: "Category", value: "Event Ticket" },
      { trait_type: "Platform", value: "Solana Ticket Platform" },
    ],
    properties: {
      files: [{ uri: imageUrl, type: "image/png" }],
      category: "image",
    },
  };
}
