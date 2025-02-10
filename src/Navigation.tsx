import React from "react";
import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router";

// Screens
import LoginPage from "@/app/login/page";
import DashboardPage from "@/app/dashboard/page";

const isAuthenticated = (): boolean => {
  return !!localStorage.getItem("authToken");
};

// Protected Route Component
const ProtectedRoute: React.FC<{ element: JSX.Element }> = ({ element }) => {
  return isAuthenticated() ? element : <Navigate to="/login" />;
};

// Public Route (Prevents logged-in users from accessing login)
const PublicRoute: React.FC<{ element: JSX.Element }> = ({ element }) => {
  return !isAuthenticated() ? element : <Navigate to="/home" />;
};

export const Navigation: React.FC = () => {
  return (
    <Router>
      <Routes>
        {/* Public Route: Redirect if already authenticated */}
        <Route path="/login" element={<PublicRoute element={<LoginPage />} />} />
        
        {/* Protected Route: Redirect to Login if not authenticated */}
        <Route path="/home" element={<ProtectedRoute element={<DashboardPage />} />} />

        {/* Default route: Redirect unknown paths to login */}
        <Route path="*" element={<Navigate to="/login" />} />
      </Routes>
    </Router>
  );
};
