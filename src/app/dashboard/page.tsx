import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Separator } from "@/components/ui/separator";
import { SidebarInset, SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";

// Import all possible modules
import SocialMediaMain from "@/app/dashboard/modules/social-media/social-media-main";
import ClientsMain from "@/app/dashboard/modules/clients/clients-main";
import FinancialsMain from "@/app/dashboard/modules/financials/financials-main";
import PermissionsMain from "@/app/dashboard/modules/permissions/permissions-main";
import SettingsMain from "@/app/dashboard/modules/settings/settings-main";
import { useState } from "react";

export default function DashboardPage() {
  const [activePage, setActivePage] = useState("clients"); // Default Page
  
  const pageComponents: Record<string, JSX.Element> = {
    clients: <ClientsMain />,
    financials: <FinancialsMain />,
    "social-media": <SocialMediaMain />,
    permissions: <PermissionsMain />,
    settings: <SettingsMain />,
  };

  return (
    <SidebarProvider>
      <AppSidebar onNavigate={setActivePage} />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2">
          <div className="flex items-center gap-2 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem className="hidden md:block">
                  <BreadcrumbLink onClick={() => setActivePage("clients")}>Dashboard</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator className="hidden md:block" />
                <BreadcrumbItem>
                  <BreadcrumbPage>
                    {activePage}
                  </BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </div>
        </header>
        {/* Define routes inside the dashboard */}
        <div className="flex flex-1 flex-col gap-4 p-4 pt-0" id="main-section">
          {pageComponents[activePage] || <ClientsMain />}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
