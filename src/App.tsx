import "@/input.css";
import { Navigation } from "./Navigation";
import { ThemeProvider } from "./components/theme-provider";
import { invoke } from "@tauri-apps/api";

function App() {
  invoke('greet', { name: 'World' })
  // `invoke` returns a Promise
  .then((response) => console.log(response))

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <Navigation />
    </ThemeProvider>
  );
}

export default App;
