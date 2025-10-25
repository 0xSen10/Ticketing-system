import { useRouter } from "next/router";

interface EventCardProps {
  id: string;
  title: string;
  price: number;
}

export default function EventCard({ id, title, price }: EventCardProps) {
  const router = useRouter();

  return (
    <div
      onClick={() => router.push(`/event/${id}`)}
      className="cursor-pointer bg-white shadow-md rounded-2xl p-4 hover:shadow-xl transition"
    >
      <h2 className="text-lg font-bold">{title}</h2>
      <p className="text-gray-500">票价: {price} SOL</p>
    </div>
  );
}
