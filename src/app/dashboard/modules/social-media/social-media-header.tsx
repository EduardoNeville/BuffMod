import { useState } from "react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu"
import { Separator } from "@/components/ui/separator"
import { Search, Plus, ChevronDown } from "lucide-react"

export default function SocialMediaHeader() {
  // Example channel list state, can be updated dynamically
  // TODO: Translate
  // TODO: Backend
  const [channels, setChannels] = useState(["General", "Marketing", "Development"])

  return (
    <header className="flex items-center justify-between bg-background px-4 py-2 border-b border-border">
      <div className="flex items-center gap-4">
        {/* Channels Dropdown or Add Channel Button */}
        {channels.length > 0 ? (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline">
                {/* TODO: Translate */}
                Channels
                <ChevronDown className="ml-2 h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-48" align="start">
              {channels.map((channel, index) => (
                <DropdownMenuItem key={index}>{channel}</DropdownMenuItem>
              ))}
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => alert("Add Channel")}>
                <Plus className="mr-2 h-4 w-4" />
                {/* TODO: Translate */}
                Add Channel
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        ) : (
          <Button variant="outline" onClick={() => alert("Add Channel")}>
            <Plus className="mr-2 h-4 w-4" />
            {/* TODO: Translate */}
            Add Channel
          </Button>
        )}

        {/* Separator */}
        <Separator orientation="vertical" className="h-6" />

        {/* Create Post Button */}
        <Button variant="default">
          <Plus className="mr-2 h-4 w-4" />
          {/* TODO: Translate */}
          Create Post
        </Button>
      </div>

      {/* Search Button */}
      <Button variant="outline">
        {/* TODO: Build */}
        <Search className="h-4 w-4" />
      </Button>
    </header>
  )
}
