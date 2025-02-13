import React from "react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

interface Event {
  date: string;
  name: string;
  icon: React.ReactNode;
  startTime: string;
  endTime: string;
}

interface DayScheduleProps {
  selectedDate: string;
  events: Event[];
}

const DaySchedule: React.FC<DayScheduleProps> = ({ selectedDate, events }) => {
  // Filter and sort the events for the selected day
  const filteredEvents = events
    .filter(event => event.date === selectedDate)
    .sort((a, b) => a.startTime.localeCompare(b.startTime));

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button >View Schedule</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[500px] bg-[var(--background)] text-[var(--foreground)]">
        <DialogHeader>
          <DialogTitle>Schedule for {selectedDate}</DialogTitle>
          <DialogDescription>Here are your events for the day.</DialogDescription>
        </DialogHeader>
        
        {filteredEvents.length > 0 ? (
          <div className="space-y-3">
            {filteredEvents.map((event, index) => (
              <div key={index} className="p-3 border rounded-lg shadow-md">
                <h3 className="text-md font-semibold flex items-center">
                  {event.icon} &nbsp; {event.name}
                </h3>
                <p className="text-sm text-[var(--muted-foreground)]">
                  {event.startTime} - {event.endTime}
                </p>
              </div>
            ))}
          </div>
        ) : (
          <p className="text-center text-sm">No events scheduled for this day.</p>
        )}
      </DialogContent>
    </Dialog>
  );
};

export default DaySchedule;
