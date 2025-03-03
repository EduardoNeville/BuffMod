import { useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Client } from "@/app/dashboard/modules/clients/clients-main";
import ClientDetailsDialog from "@/app/dashboard/modules/clients/clients-details";

type ClientsTableProps = {
  clients: Client[];
  search: string;
};

export default function ClientsTable({ clients, search }: ClientsTableProps) {
  const [selectedClientId, setSelectedClientId] = useState<number | null>(null);

  // ✅ Filter Clients Based on Search Input
  const filteredClients = clients.filter(
    (client) =>
      client.name.toLowerCase().includes(search.toLowerCase()) ||
      client.email.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
            <TableHead>Email</TableHead>
            <TableHead className="text-right">Phone</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filteredClients.map((client) => (
            <TableRow key={client.id} className="cursor-pointer hover:bg-gray-100" onClick={() => setSelectedClientId(client.id)}>
              <TableCell className="font-medium">{client.name}</TableCell>
              <TableCell>{client.email}</TableCell>
              <TableCell className="text-right">{client.phone || "N/A"}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      {/* Client Details Dialog */}
      <ClientDetailsDialog clientId={selectedClientId} onClose={() => setSelectedClientId(null)} />
    </>
  );
}
