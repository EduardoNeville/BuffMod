import React from "react";
import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router";

// Auth
import LoginPage from "@/app/auth/login/page";
import { WelcomePage } from "@/app/auth/welcome/page";
import { CreateOrganizationPage } from "./app/auth/create-organisation/page";
import { InviteLoginPage } from "./app/auth/invite-login/page";

// Home
import DashboardPage from "@/app/dashboard/page";

export const Navigation: React.FC = () => {
  return (
    <Router>
      <Routes>
        {/* Public Routes */}
        <Route path="/welcome" element={<WelcomePage />} />
        <Route path="/create-organization" element={<CreateOrganizationPage />} />
        <Route path="/invite" element={<InviteLoginPage />} />
        <Route path="/login" element={<LoginPage />} />

        {/* Protected Dashboard Route with nested routes */}
        <Route path="/dashboard/*" element={<DashboardPage/>} />

        {/* Default route: Redirect unknown paths to welcome */}
        <Route path="*" element={<Navigate to="/welcome" />} />
      </Routes>
    </Router>
  );
};
