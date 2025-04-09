import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export function SectionCards() {
  return (
    <div className="mt-4 grid grid-cols-3 gap-4 px-4 lg:px-6">
      <Card>
        <CardHeader className="relative">
          <CardDescription>Total Revenue</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums">
            $1,250.00
          </CardTitle>
        </CardHeader>
      </Card>
      <Card>
        <CardHeader className="relative">
          <CardDescription>New Customers</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums">
            1,234
          </CardTitle>
        </CardHeader>
      </Card>
      <Card>
        <CardHeader className="relative">
          <CardDescription>Active Accounts</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums">
            45,678
          </CardTitle>
        </CardHeader>
      </Card>
    </div>
  );
}
