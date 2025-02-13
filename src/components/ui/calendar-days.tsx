import React from "react";

interface CalendarDaysProps {
  currentDay: Date;
  onSelect: (day: Date) => void;
}

const CalendarDays: React.FC<CalendarDaysProps> = ({ currentDay, onSelect }) => {
  const firstDayOfMonth = new Date(currentDay.getFullYear(), currentDay.getMonth(), 1);
  const firstWeekday = firstDayOfMonth.getDay();
  const totalDays = new Date(currentDay.getFullYear(), currentDay.getMonth() + 1, 0).getDate();
  const today = new Date();
  
  const days = Array(42).fill(null).map((_, i) => {
    const dayNum = i - firstWeekday + 1;
    const date = new Date(currentDay.getFullYear(), currentDay.getMonth(), dayNum);
    
    const isValid = dayNum > 0 && dayNum <= totalDays;
    const isToday = today.toDateString() === date.toDateString();
    const isSelected = currentDay.toDateString() === date.toDateString();

    return (
      <div
        key={i}
        className={`w-10 h-10 flex items-center justify-center rounded-md
        ${isValid ? "cursor-pointer" : "opacity-30"} 
        ${isToday ? "bg-[var(--accent-color)] text-white font-bold" : "bg-[var(--secondary-color)]"}
        ${isSelected ? "border-2 border-[var(--primary-color)]" : ""}
        hover:bg-blue-300`}
        onClick={() => isValid && onSelect(date)}
      >
        {isValid ? dayNum : ""}
      </div>
    );
  });

  return <div className="grid grid-cols-7 gap-1">{days}</div>;
};

export default CalendarDays;
