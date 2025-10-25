import { useRouter } from "next/router";
import BuyTicketButton from "@/components/BuyTicketButton";
import RefundButton from "@/components/RefundButton";
import OrganizerWithdrawButton from "@/components/OrganizerWithdrawButton";

export default function EventDetail() {
  const router = useRouter();
  const { id } = router.query;

  // mock 数据，可替换为从 Solana 读取 event account
  const event = {
    id,
    title: id === "Event123" ? "Solana Conf 2025" : "Blockchain Meetup",
    price: id === "Event123" ? 1.2 : 0.3,
    total: 100,
    sold: 50,
  };

  return (
    <div className="p-10 flex flex-col items-center">
      <h1 className="text-3xl font-bold mb-2">{event.title}</h1>
      <p className="text-gray-600 mb-4">票价: {event.price} SOL</p>
      <p className="text-gray-500 mb-8">
        已售: {event.sold}/{event.total}
      </p>

      <div className="flex gap-4">
        <BuyTicketButton eventId={event.id as string} price={event.price} />
        <RefundButton eventId={event.id as string} />
        <OrganizerWithdrawButton eventId={event.id as string} />
      </div>
    </div>
  );
}
