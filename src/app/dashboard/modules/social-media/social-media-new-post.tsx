import { useState } from "react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { format } from "date-fns";
import { useToast } from "@/hooks/use-toast.ts";
import { cn } from "@/lib/utils";
import { Plus, UploadIcon, Twitter, Instagram, Linkedin, Facebook, CheckIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Separator } from "@/components/ui/separator";
import { DateTimePicker } from "@/components/ui/day-time-picker";
import { Checkbox } from "@/components/ui/checkbox";

const socialMediaPlatforms = [
  { name: "Facebook", icon: <Facebook /> },
  { name: "Twitter", icon: <Twitter />},
  { name: "Instagram", icon: <Instagram />},
  { name: "LinkedIn", icon: <Linkedin />},
];

export default function SocialMediaNewPost() {
  const [postType, setPostType] = useState("Post");
  const [content, setContent] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [scheduleDate, setScheduleDate] = useState<Date | undefined>(undefined);
  const [scheduleTime, setScheduleTime] = useState<string | undefined>(undefined);
  const [isScheduling, setIsScheduling] = useState(false);
  const { toast } = useToast();

  // TODO: Backend -> Insert into drafts
  const handleDraft = () => {
    toast({
      title: "Drafted Post",
      description: `${content}`,
    })
  };

  // TODO: Backend -> Insert into posts & scheduled cron job.
  const handlePost = () => {
    if (isScheduling && scheduleDate) {
      const selectedTime = scheduleTime ? scheduleTime : format(new Date(), "HH:mm");
      toast({
        title: "Scheduled Post",
        description: `On the ${format(scheduleDate, "PPP")} at ${selectedTime}`,
      })
    } else {
      toast({
        title: "Scheduled Post",
        description: `${content}`,
      })
    }
  };

  return (
    <Dialog>
      <DialogTrigger>
        <Button variant="default">
          <Plus className="mr-2 h-4 w-4" />
          {/* TODO: Translate */}
          Create Post
        </Button>
      </DialogTrigger>
      <DialogContent className="fixed top-1/2 left-1/2 bg-background text-foreground p-6 rounded-lg shadow-lg w-[400px] transform -translate-x-1/2 -translate-y-1/2">
        <DialogTitle className="text-lg font-semibold">Create a Post</DialogTitle>

        {/* Social Media Icons */}
        <div className="flex gap-3 mt-4">
          {socialMediaPlatforms.map(({ name, icon }) => (
            <div key={name} className="bg-secondary text-secondary-foreground p-2 rounded-lg">
             {icon} 
            </div>
          ))}
        </div>

        {/* Post Type Selection */}
        <div className="flex justify-around mt-4">
          {["Post", "Reel", "Story"].map((type) => (
            <Button
              key={type}
              className={cn(
                "px-4 py-2 rounded-lg",
                postType === type ? "bg-primary text-primary-foreground" : "bg-secondary text-secondary-foreground"
              )}
              onClick={() => setPostType(type)}
            >
              {type}
            </Button>
          ))}
        </div>

        {/* Input Section */}
        <Textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          className="w-full mt-4 p-2 border rounded-lg bg-background text-foreground"
          placeholder="Write your post..."
          rows={4}
        />

        {/* File Upload */}
        <div className="border-dashed border-2 mt-4 p-4 text-center rounded-lg bg-secondary cursor-pointer">
          <label>
            <input type="file" hidden onChange={(e) => setFile(e.target.files?.[0] || null)} />
            {file ? (
              <span className="text-primary-foreground flex items-center justify-center gap-2">
                <CheckIcon size={16} /> {file.name}
              </span>
            ) : (
              <span className="text-secondary-foreground flex flex-col items-center">
                <UploadIcon size={20} />
                Drag & Drop or Click to Upload
              </span>
            )}
          </label>
        </div>

        {/* Scheduling Options */}
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
            {/** TODO: Translate **/}
            <Checkbox onClick={() => setIsScheduling(!isScheduling)} />
            Schedule Post
          </div>
          {isScheduling && (
            <DateTimePicker
              selectedDate={scheduleDate}
              setSelectedDate={setScheduleDate}
              setSelectedTime={setScheduleTime}
            />
          )}
        </div>
        <DialogFooter>
          {/* Action Buttons */}
          <DialogClose  className="flex justify-end mt-4 gap-3">
            <Button className="bg-muted text-muted-foreground px-4 py-2 rounded-lg" onClick={handleDraft}>
              Save as Draft
            </Button>
            <Separator orientation="vertical" />
            <Button className="bg-primary text-primary-foreground px-4 py-2 rounded-lg" onClick={handlePost}>
              {isScheduling ? "Schedule" : "Post"}
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
