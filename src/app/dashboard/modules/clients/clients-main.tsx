import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ClientsHeader from "@/app/dashboard/modules/clients/clients-header";
import ClientsTable from "@/app/dashboard/modules/clients/clients-table";

// ✅ Define the Client Type
export type Client = {
  id: number;
  name: string;
  email: string;
  phone?: string;
};

export default function ClientsMain() {
  const [clients, setClients] = useState<Client[]>([]);
  const [search, setSearch] = useState(""); // Search filter

  // ✅ Fetch Clients from Rust API
  const fetchClients = () => {
    invoke<Client[]>("list_clients")
      .then((data) => setClients(data))
      .catch((error) => console.error("Error fetching clients:", error));
  };

  useEffect(() => {
    fetchClients();
  }, []);

  return (
    <div className="space-y-6">
      {/* Clients Header */}
      <ClientsHeader setSearch={setSearch} search={search} refreshClients={fetchClients} />

      {/* Clients Table */}
      <ClientsTable clients={clients} search={search} />
    </div>
  );
}
