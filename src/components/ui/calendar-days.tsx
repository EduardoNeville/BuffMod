import React, { useEffect, useState } from "react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import CalendarEvent, { CalendarEntry } from "@/components/ui/calendar-event";
import { EventKind } from "@/lib/types";
import { invoke } from "@tauri-apps/api/core";
import { Calendar, Presentation, Share2 } from "lucide-react";

interface CalendarDaysProps {
  currentDay: Date;
  onSelect: (day: Date) => void;
  eventKind?: EventKind;
}

const CalendarDays: React.FC<CalendarDaysProps> = ({ currentDay, onSelect, eventKind }) => {
  const firstDayOfMonth = new Date(currentDay.getFullYear(), currentDay.getMonth(), 1);
  const firstWeekday = firstDayOfMonth.getDay() === 0 ? 6 : firstDayOfMonth.getDay() - 1; // Adjust for Monday start
  const lastDay = new Date(currentDay.getFullYear(), currentDay.getMonth() + 1, 0);
  const totalDays = lastDay.getDate();
  const today = new Date();

  // Format date as YYYY-MM-DD in local time
  const formatDate = (date: Date) => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
  };

  const [events, setEvents] = useState<CalendarEntry[]>([]);

  const fetchEvents = () => {
    if (eventKind) {
      console.log("firstDayOfMonth", formatDate(firstDayOfMonth));
      console.log("lastDay", formatDate(lastDay));
      invoke<CalendarEntry[]>("list_events_time_kind", {
        args: {
          event_kind: eventKind,
          start_date: formatDate(firstDayOfMonth),
          end_date: formatDate(lastDay),
        },
      })
        .then((data) => {
          console.log("Fetched events:", data);
          setEvents(data);
        })
        .catch((error) => console.error("Error fetching events:", error));
    } else {
      invoke<CalendarEntry[]>("list_events")
        .then((data) => {
          console.log("Fetched events:", data);
          setEvents(data);
        })
        .catch((error) => console.error("Error fetching events:", error));
    }
  };

  // Fetch events on mount
  useEffect(() => {
    fetchEvents();
  }, []);

  // Log events when they update
  useEffect(() => {
    console.log("Events updated:", events);
  }, [events]);

  const days = Array(42)
    .fill(null)
    .map((_, i) => {
      const dayNum = i - firstWeekday + 1;
      const date = new Date(currentDay.getFullYear(), currentDay.getMonth(), dayNum);
      const formattedDate = formatDate(date);

      const isValid = dayNum > 0 && dayNum <= totalDays;
      const isToday = today.toDateString() === date.toDateString();
      //const isSelected = currentDay.toDateString() === date.toDateString();
      //${isSelected ? "border-2 border-primary text-primary" : ""}

      // Adjust filter to match date part only, assuming schedule_time may include time
      const dayEvents = events.filter((event) => {
        const eventDate = event.schedule_time.split(" ")[0]; // Extract YYYY-MM-DD
        return eventDate === formattedDate;
      });
      const hasEvents = dayEvents.length > 0;

      // Helper function to map event kind to icon with proper sizing
      const getIconForEvent = (kind: EventKind) => {
        switch (kind) {
          case EventKind.SocialMedia:
            return <Share2 className="w-4 h-4" />;
          case EventKind.Meeting:
            return <Presentation className="w-4 h-4" />;
          default:
            return <Calendar className="w-4 h-4" />;
        }
      };

      // Debugging log to check events per day
      console.log(`Day ${formattedDate}: ${dayEvents.length} events`);

      return (
        <Dialog key={i}>
          <DialogTrigger asChild>
            <div
              className={`w-full h-16 flex flex-col p-1 border rounded-md relative
                ${isValid ? "cursor-pointer" : "opacity-30"}
                ${isToday ? "bg-primary text-primary-foreground font-bold hover:text-primary-foreground" : "bg-card text-foreground hover:text-primary"}
                hover:border-2 hover:border-primary`}
              onClick={() => isValid && onSelect(date)}
            >
              <div className="text-xs">{isValid ? dayNum : ""}</div>
              {hasEvents && (
                <div className="flex items-center bg-accent text-accent-foreground text-xs mt-1 p-1 rounded-md cursor-pointer shadow-md">
                  <div className="flex space-x-1">
                    {dayEvents.map((event, i) => (
                      <div key={i}>{getIconForEvent(event.kind)}</div>
                    ))}
                  </div>
                </div>
              )}
              {dayEvents.length > 1 && (
                <div className="absolute bottom-1 right-1 bg-muted text-muted-foreground text-xs px-2 py-0.5 rounded-md">
                  +{dayEvents.length - 1}
                </div>
              )}
            </div>
          </DialogTrigger>
          <DialogContent className="w-4/5 rounded-md bg-background text-foreground">
            <DialogHeader>
              <DialogTitle>Schedule for {formattedDate}</DialogTitle>
              <DialogDescription>Here are your events for the day.</DialogDescription>
            </DialogHeader>
            {dayEvents.length > 0 ? (
              <div className="space-y-3">
                {dayEvents.map((event, index) => (
                  <CalendarEvent
                    key={`${event.schedule_time}-${index}`}
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
