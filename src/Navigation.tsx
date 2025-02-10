import React from "react";
import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router";

// Screens
import LoginPage from "@/app/login/page";
import DashboardPage from "@/app/dashboard/page";

export const Nav: React.FC<> = () => {
  return (
    <Router>
      <Routes>
        {/* Redirect to Home if logged in, otherwise show Login */}
        <Route path="/login" element={<LoginPage />} />

        {/* Redirect to Login if not logged in */}
        <Route path="/home" element={<DashboardPage />} />

        {/* Default Route */}
        <Route path="*" element={<Navigate to="/login" />} />
      </Routes>
    </Router>
  );
};
