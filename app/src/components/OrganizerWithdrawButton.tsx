import { useState } from "react";
import { useTicketProgram } from "@/lib/anchorClient";

export default function OrganizerWithdrawButton({ eventId }: { eventId: string }) {
  const [loading, setLoading] = useState(false);
  const { program, provider } = useTicketProgram();

  const handleWithdraw = async () => {
    if (!program || !provider) return alert("请先连接钱包！");
    setLoading(true);

    try {
      const tx = await program.methods
        .withdrawFunds()
        .accounts({
          organizer: provider.wallet.publicKey,
          event: eventId,
        })
        .rpc();

      alert(`提现成功！交易签名：${tx}`);
    } catch (err) {
      console.error(err);
      alert("提现失败！");
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleWithdraw}
      disabled={loading}
      className="bg-yellow-500 text-white px-4 py-2 rounded hover:bg-yellow-600 disabled:bg-gray-400"
    >
      {loading ? "提现中..." : "提现收益"}
    </button>
  );
}
