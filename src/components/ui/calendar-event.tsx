import React from "react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { EventKind } from "@/lib/types";


export interface CalendarEntry {
  kind: EventKind;
  title: string;
  schedule_time: string;
  end_time?: string;
  client_id?: number;
  description?: string;
  completed: boolean;
}

interface CalendarEventProps {
  index: number;
  dayEvent: CalendarEntry;
}

const CalendarEvent: React.FC<CalendarEventProps> = ({ index, dayEvent }) => {
  return (
    <Dialog key={index}>
      <DialogTrigger asChild>
        <div className="p-3 border rounded-lg shadow-md">
          <h3>
            {dayEvent.icon} &nbsp; {dayEvent.name}
          </h3>
          <p>
            {dayEvent.startTime} - {dayEvent.endTime}
          </p>
        </div>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {dayEvent.icon} &nbsp; {dayEvent.name}
          </DialogTitle>
          <DialogDescription>
            This action cannot be undone. This will permanently delete your account
            and remove your data from our servers.
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
};

export default CalendarEvent;


