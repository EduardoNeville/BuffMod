import React, { useState } from "react";
import { Button } from "./button";
import { ChevronLeft, ChevronRight } from "lucide-react";
import CalendarDays from "@/components/ui/calendar-days";
import { CalendarEntry } from "@/components/ui/calendar-event"; // Import Event interface

const CalendarEvents: React.FC = () => {
  const today = new Date();
  const [currentDay, setCurrentDay] = useState<Date>(today);

  // TODO: Translate
  const months = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];
  const weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

  const changeMonth = (offset: number) => {
    setCurrentDay(new Date(currentDay.getFullYear(), currentDay.getMonth() + offset, 1));
  };

  // TODO: Backend
  const sampleEvents: CalendarEntry[] = [
    { schedule_time: "2025-02-10", name: "Meeting", icon: "📅", startTime: "09:00", endTime: "10:30" },
    { schedule_time: "2025-02-10", name: "Party", icon: "🎉", startTime: "18:00", endTime: "20:00" },
    { schedule_time: "2025-02-15", name: "Dentist", icon: "🦷", startTime: "14:00", endTime: "14:45" },
    { schedule_time: "2025-02-20", name: "Flight", icon: "✈️", startTime: "06:00", endTime: "09:00" }
  ];

  return (
    <div className="w-full mx-auto bg-[var(--primary-color)] text-[var(--text-color)] rounded-lg p-5 shadow-lg">
      <div className="flex justify-between items-center mb-4">
        <Button 
          variant="outline"
          size="icon"
          onClick={() => changeMonth(-1)}
        >
          <ChevronLeft />
        </Button>
        <h2 className="text-lg font-semibold">{months[currentDay.getMonth()]} {currentDay.getFullYear()}</h2>
        <Button 
          variant="outline"
          size="icon"
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

export default CalendarEvents;
