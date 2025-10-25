// src/components/BuyTicketButton.tsx
import { useState } from "react";
import { useTicketProgram } from "@/lib/anchorClient";
import { PublicKey, SystemProgram } from "@solana/web3.js";

export default function BuyTicketButton({ eventId, price }: { eventId: string; price: number }) {
  const [loading, setLoading] = useState(false);
  const { program, provider } = useTicketProgram();

  const handleBuy = async () => {
    if (!program || !provider) return alert("请先连接钱包！");
    setLoading(true);

    try {
      // 假设合约有一个 buy_ticket 方法
      const tx = await program.methods
        .buyTicket(new PublicKey(eventId))
        .accounts({
          buyer: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("购买成功:", tx);
      alert(`购票成功！交易签名：${tx}`);
    } catch (err) {
      console.error(err);
      alert("购买失败！");
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleBuy}
      disabled={loading}
      className="px-5 py-2 bg-blue-600 text-white rounded-xl hover:bg-blue-700 disabled:bg-gray-400"
    >
      {loading ? "购买中..." : `购买 (${price} SOL)`}
    </button>
  );
}
