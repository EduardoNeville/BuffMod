import React, { useState } from "react";
import CalendarDays from "@/components/ui/calendar-days";

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

  return (
    <div className="max-w-md mx-auto bg-[var(--primary-color)] text-[var(--text-color)] rounded-lg p-5 shadow-lg mt-10">
      <div className="flex justify-between items-center mb-4">
        <button 
          className="bg-[var(--accent-color)] px-3 py-1 rounded-md hover:bg-blue-600" 
          onClick={() => changeMonth(-1)}
        >Previous</button>
        <h2 className="text-lg font-semibold">{months[currentDay.getMonth()]} {currentDay.getFullYear()}</h2>
        <button 
          className="bg-[var(--accent-color)] px-3 py-1 rounded-md hover:bg-blue-600" 
          onClick={() => changeMonth(1)}
        >Next</button>
      </div>
      <button 
        className="bg-[var(--accent-color)] w-full py-2 my-2 text-sm rounded-md hover:bg-blue-600" 
        onClick={() => setCurrentDay(today)}
      >
        Today
      </button>
      <div className="grid grid-cols-7 gap-1 mb-2 font-semibold">
        {weekdays.map(day => <div key={day} className="text-center">{day}</div>)}
      </div>
      <CalendarDays currentDay={currentDay} onSelect={setCurrentDay} />
    </div>
  );
};

export default Calendar;
