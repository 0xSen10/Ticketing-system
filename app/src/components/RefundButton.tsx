// src/components/RefundButton.tsx
import { useState } from "react";
import { useTicketProgram } from "@/lib/anchorClient";
import { PublicKey } from "@solana/web3.js";

export default function RefundButton({ eventId }: { eventId: string }) {
  const [loading, setLoading] = useState(false);
  const { program, provider } = useTicketProgram();

  const handleRefund = async () => {
    if (!program || !provider) return alert("请先连接钱包！");
    setLoading(true);

    try {
      // 假设合约有一个 refund_ticket 方法
      const tx = await program.methods
        .refundTicket(new PublicKey(eventId))
        .accounts({
          buyer: provider.wallet.publicKey,
        })
        .rpc();

      console.log("退票成功:", tx);
      alert(`退票成功！交易签名：${tx}`);
    } catch (err) {
      console.error(err);
      alert("退票失败！");
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleRefund}
      disabled={loading}
      className="px-5 py-2 bg-red-600 text-white rounded-xl hover:bg-red-700 disabled:bg-gray-400"
    >
      {loading ? "退票中..." : "退票"}
    </button>
  );
}
