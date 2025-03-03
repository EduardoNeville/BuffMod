import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useToast } from "@/hooks/use-toast";
import { Client } from "@/app/dashboard/modules/clients/clients-main";

type ClientDetailsDialogProps = {
  clientId: number | null;
  onClose: () => void;
};

export default function ClientDetailsDialog({ clientId, onClose }: ClientDetailsDialogProps) {
  const [client, setClient] = useState<Client | null>(null);
  const { toast } = useToast();

  useEffect(() => {
    if (clientId) {
      invoke<Client>("get_client_by_id", { clientId })
        .then(setClient)
        .catch(() => {
          toast({
            title: "Error",
            description: "Failed to fetch client details.",
          });
        });
    }
  }, [clientId]);

  return (
    <Dialog open={!!clientId} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Client Details</DialogTitle>
          <DialogDescription>View detailed information about the client</DialogDescription>
        </DialogHeader>

        {client ? (
          <div className="space-y-4">
            <p><strong>Name:</strong> {client.name}</p>
            <p><strong>Email:</strong> {client.email}</p>
            <p><strong>Phone:</strong> {client.phone || "N/A"}</p>
          </div>
        ) : (
          <p>Loading client details...</p>
        )}

        <Button onClick={onClose} variant="outline">Close</Button>
      </DialogContent>
    </Dialog>
  );
}
