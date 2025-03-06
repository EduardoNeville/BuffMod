import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { writeFile } from "@tauri-apps/plugin-fs";
import * as path from '@tauri-apps/api/path';
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
import { 
  Plus, UploadIcon, Twitter, Instagram, Linkedin, Facebook, CheckIcon, ImageIcon, VideoIcon, FileIcon
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Separator } from "@/components/ui/separator";
import { DateTimePicker } from "@/components/ui/day-time-picker";
import { Checkbox } from "@/components/ui/checkbox";

const socialMediaPlatforms = [
  { name: "Facebook", icon: Facebook },
  { name: "Twitter", icon: Twitter },
  { name: "Instagram", icon: Instagram },
  { name: "LinkedIn", icon: Linkedin },
];

export default function SocialMediaNewPost({ fetchPosts }: { fetchPosts: () => void }) {
  const [selectedPlatforms, setSelectedPlatforms] = useState<string[]>([]);
  const [content, setContent] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [filePreview, setFilePreview] = useState<JSX.Element | null>(null);
  const [filePath, setFilePath] = useState<string | null>(null);
  const [scheduleDate, setScheduleDate] = useState<Date | undefined>(undefined);
  const [scheduleTime, setScheduleTime] = useState<string | undefined>(undefined);
  const [isScheduling, setIsScheduling] = useState(false);

  const { toast } = useToast();

  /** ✅ Toggle platform selection */
  const handlePlatformClick = (platform: string) => {
    setSelectedPlatforms((prev) =>
      prev.includes(platform) ? prev.filter((p) => p !== platform) : [...prev, platform]
    );
  };

  /* Update file **/
  useEffect(() => {
    if (file) {
      const fileType = file.type;
      if (fileType.startsWith("image/")) {
        setFilePreview(<ImageIcon />);
      } else if (fileType.startsWith("video/")){
        setFilePreview(<VideoIcon />);
      } else {
        setFilePreview(<FileIcon />);
      } 
    } else {
      setFilePreview(null);
    }
  }, [file]);

  /** ✅ Handle File Upload */
  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    console.log("Handle FileUpload...")
    if (!event.target.files?.length) {
      console.log("No length")
      return
    }

    const uploadedFile = event.target.files[0]
    setFile(uploadedFile);

    try {
      const tempDir = await path.dataDir();
      const tempFilePath = `buffmod/tmp/temp_upload_${Date.now()}.${uploadedFile.name.split('.').pop()}`;

      console.log("data dir: ", tempDir);
      const fileArrayBuff = await uploadedFile.arrayBuffer();
      const fullPath = `${tempDir}/${tempFilePath}`;

      await writeFile(
        tempFilePath,
        new Uint8Array(fileArrayBuff),
        { baseDir: path.BaseDirectory.Data }
      );

      setFilePath(fullPath)
    } catch (error) {
      console.log("Error saving file: ", error);
      alert("Failed to process the file");
      setFile(null);
    }
  };

  /** ✅ Handle Post Creation */
  const handlePost = async (draftStatus: boolean) => {
    const postStatus = draftStatus ? "Drafted" : isScheduling ? "Scheduled" : "Posted";

    console.log("Final Post Status:", postStatus);

    try {
      console.log("file_path: ", filePath);
      await invoke("schedule_social_post", {
        args: {
          post: {
            platform: selectedPlatforms.join("::"),
            content,
            status: postStatus,
          },
          schedule_time: isScheduling && scheduleDate ? format(scheduleDate, "yyyy-MM-dd HH:mm:ss") : "",
          file_path: filePath,
        }
      });

      // Deletion of temp file done in the backend
      const newFilePath = await invoke('retrieve_post_file', { socialMediaPostId: 18 });
      console.log(`File saved at: ${newFilePath}`);

      toast({ title: "Success!", description: `Post ${postStatus.toLowerCase()} successfully!` });

      // Reset states
      setSelectedPlatforms([]);
      setContent("");
      setFile(null);
      setFilePreview(null);
      setScheduleDate(undefined);
      setScheduleTime(undefined);
      setIsScheduling(false);
      fetchPosts();
    } catch (error) {
      console.error(error);
      toast({ title: "Error", description: "There was an issue scheduling the post." });
    }
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="default">
          <Plus className="mr-2 h-4 w-4" />
          Create Post
        </Button>
      </DialogTrigger>
      <DialogContent className="fixed top-1/2 left-1/2 bg-background text-foreground p-6 rounded-lg shadow-lg w-[400px] transform -translate-x-1/2 -translate-y-1/2">
        <DialogTitle className="text-lg font-semibold">Create a Post</DialogTitle>

        {/* ✅ Social Media Selection */}
        <div className="flex gap-3 mt-4">
          {socialMediaPlatforms.map(({ name, icon: Icon }) => (
            <button
              key={name}
              onClick={() => handlePlatformClick(name)}
              className={cn(
                "p-2 rounded-lg transition-all",
                selectedPlatforms.includes(name) ? "bg-primary text-primary-foreground" : "bg-secondary text-secondary-foreground"
              )}
            >
              <Icon size={24} />
            </button>
          ))}
        </div>

        {/* ✅ Post Content */}
        <Textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          className="w-full mt-4 p-2 border rounded-lg bg-background text-foreground"
          placeholder="Write your post..."
          rows={4}
        />

        {/* ✅ File Upload */}
        <div className="border-dashed border-2 mt-4 p-4 text-center rounded-lg bg-secondary cursor-pointer">
          <label>
            <input type="file" hidden onChange={handleFileUpload} />
            {file ? (
              <span className="text-primary-foreground flex items-center justify-center gap-2">
                <CheckIcon size={16} /> {file.name} {filePreview}
              </span>
            ) : (
              <span className="text-secondary-foreground flex flex-col items-center">
                <UploadIcon size={20} />
                Drag & Drop or Click to Upload
              </span>
            )}
          </label>
        </div>

        {/* ✅ Scheduling Options */}
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
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

        {/* ✅ Action Buttons */}
        <DialogFooter>
          <DialogClose className="flex justify-end mt-4 gap-3">
            <Button 
              className="bg-muted text-muted-foreground px-4 py-2 rounded-lg" 
              onClick={async () => {handlePost(true)}}>
              Save as Draft
            </Button>
            <Separator orientation="vertical" />
            <Button className="bg-primary text-primary-foreground px-4 py-2 rounded-lg" onClick={async () => {handlePost(false)}}>
              {isScheduling ? "Schedule" : "Post"}
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
