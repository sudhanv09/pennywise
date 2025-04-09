import { Separator } from "@/components/ui/separator";
import { SidebarTrigger } from "@/components/ui/sidebar";
import { createFileRoute } from "@tanstack/react-router";
import { SectionCards } from "@/components/tx-cards";
import { TxTable } from "@/components/tx-table";

export const Route = createFileRoute("/app/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger className="-ml-1" />
        <Separator orientation="vertical" className="mr-2 h-4" />
        <h1 className="text-base font-medium">Dashboard</h1>
      </header>
      <section>
        <SectionCards />
      </section>
      <section>
        <TxTable />
      </section>
    </div>
  );
}
