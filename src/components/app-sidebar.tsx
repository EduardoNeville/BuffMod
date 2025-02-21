import * as React from "react"
import {
  AudioWaveform,
  BookUser,
  Command,
  GalleryVerticalEnd,
  Settings,
  Share2,
  Shield,
  Wallet,
} from "lucide-react"

import { NavMain } from "@/components/nav-main"
import { NavUser } from "@/components/nav-user"
import { TeamSwitcher } from "@/components/team-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar"
import { ModeToggle } from "@/components/mode-toggle"

// This is sample data.
const data = {
  user: {
    name: "shadcn",
    email: "m@example.com",
    avatar: "/avatars/shadcn.jpg",
  },
  teams: [
    {
      name: "Acme Inc",
      logo: GalleryVerticalEnd,
      plan: "Enterprise",
    },
    {
      name: "Acme Corp.",
      logo: AudioWaveform,
      plan: "Startup",
    },
    {
      name: "Evil Corp.",
      logo: Command,
      plan: "Free",
    },
  ],
  navMain: [
    {
      title: "Clients",
      url: "clients",
      icon: BookUser,
      isActive: true,
      items: [
        {
          title: "List",
          url: "clients",
        },
        {
          title: "Starred",
          url: "#",
        },
        {
          title: "Settings",
          url: "#",
        },
      ],
    },
    {
      title: "Financials",
      url: "financials",
      icon: Wallet,
      items: [
        {
          title: "Invoices",
          url: "financials",
        },
        {
          title: "Expenses",
          url: "#",
        },
      ],
    },
    {
      title: "Social Media",
      url: "social-media",
      icon: Share2,
      items: [
        {
          title: "Publish",
          url: "social-media",
        },
        {
          title: "Analytics",
          url: "#",
        },
      ],
    },
    {
      title: "Permissions",
      url: "permissions",
      icon: Shield,
      items: [
        {
          title: "Team",
          url: "#",
        },
        {
          title: "Manage",
          url: "permissions",
        },
      ],
    },
    {
      title: "Settings",
      url: "settings",
      icon: Settings,
      items: [
        {
          title: "General",
          url: "settings",
        },
        {
          title: "Billing",
          url: "#",
        },
        {
          title: "Limits",
          url: "#",
        },
      ],
    },
  ],
}

export function AppSidebar({
  onNavigate,
  ...props
}: {
  onNavigate: (page: string) => void;
} & React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <TeamSwitcher teams={data.teams} />
      </SidebarHeader>
      <SidebarContent>
        <NavMain onNavigate={onNavigate} items={data.navMain} />
      </SidebarContent>
      <SidebarFooter>
        <ModeToggle />
        <NavUser user={data.user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
