import React, { useState } from "react";
import { Button } from "./button";
import { ChevronLeft, ChevronRight } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"

interface Event {
  date: string; // Format: "YYYY-MM-DD"
  name: string;
  icon: React.ReactNode;
  startTime: string; // Format: "HH:mm"
  endTime: string;   // Format: "HH:mm"
}

interface CalendarDaysProps {
  currentDay: Date;
  onSelect: (day: Date) => void;
  events: Event[];
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
        <Dialog>
          <DialogTrigger asChild>
            <div
              key={i}
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
              <DialogTitle>Schedule for {date.toISOString().split("T")[0]}</DialogTitle>
              <DialogDescription>Here are your events for the day.</DialogDescription>
            </DialogHeader>
            
            {dayEvents.length > 0 ? (
              <div className="space-y-3">
                {dayEvents.map((event, index) => (
                  <div key={index} className="p-3 border rounded-lg shadow-md">
                    <h3>
                      {event.icon} &nbsp; {event.name}
                    </h3>
                    <p>
                      {event.startTime} - {event.endTime}
                    </p>
                  </div>
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

const Calendar: React.FC = () => {
  const today = new Date();
  const [currentDay, setCurrentDay] = useState<Date>(today);

  const months = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];
  const weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

  const changeMonth = (offset: number) => {
    setCurrentDay(new Date(currentDay.getFullYear(), currentDay.getMonth() + offset, 1));
  };

  const sampleEvents: Event[] = [
    { date: "2025-02-10", name: "Meeting", icon: "📅", startTime: "09:00", endTime: "10:30" },
    { date: "2025-02-10", name: "Party", icon: "🎉", startTime: "18:00", endTime: "20:00" },
    { date: "2025-02-15", name: "Dentist", icon: "🦷", startTime: "14:00", endTime: "14:45" },
    { date: "2025-02-20", name: "Flight", icon: "✈️", startTime: "06:00", endTime: "09:00" }
  ];

  return (
    <div className="w-full mx-auto bg-[var(--primary-color)] text-[var(--text-color)] rounded-lg p-5 shadow-lg mt-10">
      <div className="flex justify-between items-center mb-4">
        <Button 
          variant= "outline" size="icon"
          onClick={() => changeMonth(-1)}
        >
          <ChevronLeft />
        </Button>
        <h2 className="text-lg font-semibold">{months[currentDay.getMonth()]} {currentDay.getFullYear()}</h2>
        <Button 
          variant= "outline" size="icon"
          onClick={() => changeMonth(1)}
        >
          <ChevronRight />
        </Button>
      </div>
      <div className="grid grid-cols-7 gap-1 mb-2 font-semibold">
        {weekdays.map(day => <div key={day} className="text-center">{day}</div>)}
      </div>
      <CalendarDays currentDay={currentDay} onSelect={setCurrentDay} events={sampleEvents} />
    </div>
  );
};

export default Calendar;
