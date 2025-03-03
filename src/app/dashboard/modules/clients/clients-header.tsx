import { Command, CommandInput, CommandList, CommandEmpty } from "@/components/ui/command";
import { Search } from "lucide-react";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import NewClientDialog from "@/app/dashboard/modules/clients/clients-new-client";
import { Client } from "@/app/dashboard/modules/clients/clients-main";

type ClientsHeaderProps = {
  setSearch: (search: string) => void;
  search: string;
  refreshClients: () => void;
};

export default function ClientsHeader({ setSearch, search, refreshClients }: ClientsHeaderProps) {
  return (
    <header className="flex items-center justify-between bg-background px-4 py-2 border-b border-border">
      {/* Search Bar */}
      <Dialog>
        <DialogTrigger>
          <Button>
            <Search />
          </Button>
        </DialogTrigger>
        <DialogContent>
          <Command>
            <CommandInput placeholder="Search clients..." value={search} onValueChange={setSearch} />
            <CommandList>
              <CommandEmpty>No results found.</CommandEmpty>
            </CommandList>
          </Command>
        </DialogContent>
      </Dialog>

      {/* Add New Client */}
      <NewClientDialog refreshClients={refreshClients} />
    </header>
  );
}
