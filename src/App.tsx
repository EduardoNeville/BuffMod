import "@/input.css";
import { Navigation } from "@/Navigation";
import { ThemeProvider } from "@/components/theme-provider";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <Navigation />
    </ThemeProvider>
  );
}

export default App;
