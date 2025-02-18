import { useState } from "react";
import { useNavigate } from "react-router";
import { motion } from "framer-motion";
import { Card, CardContent } from "@/components/ui/card";

export function WelcomePage() {
  const navigate = useNavigate();
  const [selected] = useState(false);

  return (
    <motion.div 
      className="flex flex-col gap-6 justify-center items-center h-screen"
      initial={{ opacity: 0, y: 30 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      <h1 className="text-2xl font-bold">Welcome to BuffMod</h1>
      <p className="text-muted-foreground">Get started by choosing an option.</p>

      <motion.div 
        className={`grid gap-6 w-full max-w-md transition-all duration-300 ${
          selected ? "opacity-0 pointer-events-none" : "opacity-100"
        }`}
      >

        <Card onClick={() => navigate("/create-organization")} className="cursor-pointer hover:shadow-md transition">
          <CardContent className="p-6 text-center">
            <h2 className="font-semibold text-lg">Create Organization</h2>
            <p className="text-sm text-muted-foreground">Start your own organization as an admin.</p>
          </CardContent>
        </Card>

        <Card onClick={() => navigate("/login")} className="cursor-pointer hover:shadow-md transition">
          <CardContent className="p-6 text-center">
            <h2 className="font-semibold text-lg">Login</h2>
            <p className="text-sm text-muted-foreground">Already have an account? Log in here.</p>
          </CardContent>
        </Card>

        <Card onClick={() => navigate("/invite")} className="cursor-pointer hover:shadow-md transition">
          <CardContent className="p-6 text-center">
            <h2 className="font-semibold text-lg">Use Invite Link</h2>
            <p className="text-sm text-muted-foreground">Join an existing organization with an invite.</p>
          </CardContent>
        </Card>

      </motion.div>
    </motion.div>
  );
}
