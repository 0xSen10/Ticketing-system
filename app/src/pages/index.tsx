// app/src/pages/index.tsx
import Link from "next/link";

export default function Home() {
  const mockEvents = [
    { id: "Event123", name: "Solana Conf 2025", price: 1.2 },
    { id: "Event456", name: "Blockchain Meetup", price: 0.3 },
  ];

  return (
    <div className="max-w-2xl mx-auto mt-12 p-4">
      <h1 className="text-3xl font-bold mb-6">🎟 Solana Ticket 平台</h1>
      {mockEvents.map((e) => (
        <Link
          key={e.id}
          href={`/event/${e.id}`}
          className="block mb-4 p-4 bg-white shadow rounded-xl hover:shadow-lg transition"
        >
          <div className="text-xl font-semibold">{e.name}</div>
          <div className="text-gray-500 text-sm">票价：{e.price} SOL</div>
        </Link>
      ))}
    </div>
  );
}
