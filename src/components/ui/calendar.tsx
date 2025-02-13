import React, { useState } from "react";

interface Event {
  date: string; // Format: "YYYY-MM-DD"
  name: string;
  icon: React.ReactNode;
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
  const weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

  const changeMonth = (offset: number) => {
    setCurrentDay(new Date(currentDay.getFullYear(), currentDay.getMonth() + offset, 1));
  };

  const sampleEvents: Event[] = [
    { date: "2025-02-10", name: "Meeting", icon: "📅" },
    { date: "2025-02-10", name: "Party", icon: "🎉" },
    { date: "2025-02-15", name: "Dentist", icon: "🦷" },
    { date: "2025-02-20", name: "Flight", icon: "✈️" }
  ];

  return (
    <div className="w-full mx-auto bg-[var(--primary-color)] text-[var(--text-color)] rounded-lg p-5 shadow-lg mt-10">
      <div className="flex justify-between items-center mb-4">
        <button 
          className="bg-[var(--accent-color)] px-3 py-1 rounded-md hover:bg-blue-600" 
          onClick={() => changeMonth(-1)}
        >
          Previous
        </button>
        <h2 className="text-lg font-semibold">{months[currentDay.getMonth()]} {currentDay.getFullYear()}</h2>
        <button 
          className="bg-[var(--accent-color)] px-3 py-1 rounded-md hover:bg-blue-600" 
          onClick={() => changeMonth(1)}
        >
          Next
        </button>
      </div>
      <div className="grid grid-cols-7 gap-1 mb-2 font-semibold">
        {weekdays.map(day => <div key={day} className="text-center">{day}</div>)}
      </div>
      <CalendarDays currentDay={currentDay} onSelect={setCurrentDay} events={sampleEvents} />
    </div>
  );
};

export default Calendar;
