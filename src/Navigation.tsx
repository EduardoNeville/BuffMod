import React from "react";
import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router";

// Auth
import LoginPage from "@/app/auth/login/page";
import { WelcomePage } from "@/app/auth/welcome/page";
import { CreateOrganizationPage } from "./app/auth/create-organisation/page";
import { InviteLoginPage } from "./app/auth/invite-login/page";

// Home
import DashboardPage from "@/app/dashboard/page";

const isAuthenticated = (): boolean => {
  return !!localStorage.getItem("authToken");
};

// Protected Route Component
const ProtectedRoute: React.FC<{ element: JSX.Element }> = ({ element }) => {
  return isAuthenticated() ? element : <Navigate to="/welcome" />;
};

// Public Route (Prevents logged-in users from accessing login)
const PublicRoute: React.FC<{ element: JSX.Element }> = ({ element }) => {
  return !isAuthenticated() ? element : <Navigate to="/dashboard" />;
};

export const Navigation: React.FC = () => {
  return (
    <Router>
      <Routes>
        {/* Public Routes */}
        <Route path="/welcome" element={<PublicRoute element={<WelcomePage />} />} />
        <Route path="/create-organization" element={<PublicRoute element={<CreateOrganizationPage />} />} />
        <Route path="/invite" element={<PublicRoute element={<InviteLoginPage />} />} />
        <Route path="/login" element={<PublicRoute element={<LoginPage />} />} />

        {/* Protected Dashboard Route with nested routes */}
        <Route path="/dashboard/*" element={<ProtectedRoute element={<DashboardPage />} />} />

        {/* Default route: Redirect unknown paths to welcome */}
        <Route path="*" element={<Navigate to="/welcome" />} />
      </Routes>
    </Router>
  );
};
