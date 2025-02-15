import React from "react";
import ReactDOM from "react-dom/client";
import { Toaster } from "./components/ui/toaster";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
    <Toaster />
  </React.StrictMode>,
);
