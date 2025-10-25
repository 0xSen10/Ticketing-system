import { useState } from "react";
import { useTicketProgram } from "@/lib/anchorClient";

export default function CreateEventForm() {
  const { program, provider } = useTicketProgram();
  const [title, setTitle] = useState("");
  const [price, setPrice] = useState("");
  const [supply, setSupply] = useState("");
  const [timeLock, setTimeLock] = useState("");

  const handleCreate = async () => {
    if (!program || !provider) return alert("请先连接钱包！");

    try {
      const tx = await program.methods
        .createEvent(title, parseFloat(price), parseInt(supply), parseInt(timeLock))
        .accounts({
          organizer: provider.wallet.publicKey,
        })
        .rpc();

      alert(`活动上架成功！交易签名：${tx}`);
    } catch (err) {
      console.error(err);
      alert("上架失败！");
    }
  };

  return (
    <div className="bg-white shadow-xl rounded-2xl p-8 w-[400px] flex flex-col gap-4">
      <input
        className="border p-2 rounded"
        placeholder="活动标题"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <input
        className="border p-2 rounded"
        placeholder="票价 (SOL)"
        value={price}
        onChange={(e) => setPrice(e.target.value)}
      />
      <input
        className="border p-2 rounded"
        placeholder="总票数"
        value={supply}
        onChange={(e) => setSupply(e.target.value)}
      />
      <input
        className="border p-2 rounded"
        placeholder="解锁时间戳 (Unix)"
        value={timeLock}
        onChange={(e) => setTimeLock(e.target.value)}
      />

      <button
        onClick={handleCreate}
        className="bg-green-600 text-white py-2 rounded hover:bg-green-700"
      >
        上架活动
      </button>
    </div>
  );
}
