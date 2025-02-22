import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api"
import {
  Command,
  CommandInput,
  CommandList,
  CommandItem,
  CommandEmpty,
} from "@/components/ui/command"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { Search } from "lucide-react";
import NewClientDialog from "./clients-new-client"
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"

// ✅ Define the Client Type
type Client = {
  id: number
  name: string
  email: string
  phone?: string
}

export default function ClientsMain() {
  const [clients, setClients] = useState<Client[]>([])
  const [search, setSearch] = useState("") // Search filter
  
  // ✅ Fetch Clients from Rust API
  useEffect(() => {
    invoke<Client[]>("list_clients")
      .then((data) => setClients(data))
      .catch((error) => {
        console.error("Error fetching clients:", error)
      })
  }, [])

  // ✅ Filter Clients Based on Search Input
  const filteredClients = clients.filter(
    (client) =>
      client.name.toLowerCase().includes(search.toLowerCase()) ||
      client.email.toLowerCase().includes(search.toLowerCase())
  )

  return (
    <div className="space-y-6">
      {/*  Header with Search and "New Client" Button */}
      <header className="flex items-center justify-between bg-background px-4 py-2 border-b border-border">
        {/* Search Bar Using Command Component */}
        <Dialog>
          <DialogTrigger>
            <Button> <Search/></Button>
          </DialogTrigger>
          <DialogContent>
            <Command>
              <CommandInput
                placeholder="Search clients..."
                value={search}
                onValueChange={(value) => setSearch(value)}
              />
              <CommandList>
                <CommandEmpty>No results found.</CommandEmpty>
                {filteredClients.map((client) => (
                  <CommandItem key={client.id}>{client.name}</CommandItem>
                ))}
              </CommandList>
            </Command>
          </DialogContent>
        </Dialog>

        {/* Button to Add New Client */}
        <NewClientDialog />
      </header>

      {/*   Client List Table */}
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
            <TableRow key={client.id}>
              <TableCell className="font-medium">{client.name}</TableCell>
              <TableCell>{client.email}</TableCell>
              <TableCell className="text-right">{client.phone || "N/A"}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
