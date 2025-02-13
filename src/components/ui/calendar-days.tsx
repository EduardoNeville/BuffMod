import React from "react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import CalendarEvent, { CalendarEntry } from "@/components/ui/calendar-event"; // Import Event interface

interface CalendarDaysProps {
  currentDay: Date;
  onSelect: (day: Date) => void;
  events: CalendarEntry[];
}

const CalendarDays: React.FC<CalendarDaysProps> = ({ currentDay, onSelect, events }) => {
  const firstDayOfMonth = new Date(currentDay.getFullYear(), currentDay.getMonth(), 1);
  const firstWeekday = firstDayOfMonth.getDay();
  const totalDays = new Date(currentDay.getFullYear(), currentDay.getMonth() + 1, 0).getDate();
  const today = new Date();

  const formatDate = (date: Date) => date.toISOString().split("T")[0];

  const days = Array(42)
    .fill(null)
    .map((_, i) => {
      const dayNum = i - firstWeekday + 1;
      const date = new Date(currentDay.getFullYear(), currentDay.getMonth(), dayNum);
      const formattedDate = formatDate(date);

      const isValid = dayNum > 0 && dayNum <= totalDays;
      const isToday = today.toDateString() === date.toDateString();
      const isSelected = currentDay.toDateString() === date.toDateString();

      const dayEvents = events.filter((event) => event.date === formattedDate);
      const hasEvents = dayEvents.length > 0;

      return (
        <Dialog key={i}>
          <DialogTrigger asChild>
            <div
              className={`w-full h-16 flex flex-col p-1 border rounded-md relative
                ${isValid ? "cursor-pointer" : "opacity-30"} 
                ${isToday ? "bg-[var(--accent-color)] text-white font-bold" : "bg-[var(--secondary-color)]"}
                ${isSelected ? "border-2 border-[var(--primary-color)]" : ""}
                hover:bg-blue-300`}
              onClick={() => isValid && onSelect(date)}
            >
              <div className="text-xs">{isValid ? dayNum : ""}</div>
              {hasEvents && (
                <div className="flex items-center bg-blue-100 text-xs mt-1 p-1 rounded-md cursor-pointer shadow-md">
                  {dayEvents[0].icon} &nbsp; {dayEvents[0].name}
                </div>
              )}

              {dayEvents.length > 1 && (
                <div className="absolute bottom-1 right-1 bg-gray-400 text-white text-xs px-2 py-0.5 rounded-md">
                  +{dayEvents.length - 1}
                </div>
              )}
            </div>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              {/** TODO: Translate **/}
              <DialogTitle>Schedule for {formattedDate}</DialogTitle>
              <DialogDescription>Here are your events for the day.</DialogDescription>
            </DialogHeader>

            {/** TODO: Translate **/}
            {dayEvents.length > 0 ? (
              <div className="space-y-3">
                {dayEvents.map((event, index) => (
                  <CalendarEvent 
                    key={`${event.date}-${index}`}
                    index={index}
                    dayEvent={event} 
                  />
                ))}
              </div>
            ) : (
              <p>No events scheduled for this day.</p>
            )}
          </DialogContent>
        </Dialog>
      );
    });

  return <div className="grid grid-cols-7 gap-1">{days}</div>;
};

export default CalendarDays;
