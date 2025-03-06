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
            {dayEvent.title}
          </h3>
          <p>
            {dayEvent.schedule_time}
          </p>
        </div>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {dayEvent.title}
          </DialogTitle>
          <DialogDescription>
            Event with that title
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
};

export default CalendarEvent;


