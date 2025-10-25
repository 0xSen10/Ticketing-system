import CreateEventForm from "@/components/CreateEventForm";

export default function CreatePage() {
  return (
    <div className="p-10 flex flex-col items-center">
      <h1 className="text-3xl font-bold mb-8">上架新活动</h1>
      <CreateEventForm />
    </div>
  );
}
