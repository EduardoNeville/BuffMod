import { useState } from "react";
import { useNavigate } from "react-router";
import { motion } from "framer-motion";
import { signIn } from "@/lib/auth";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Form, FormControl, FormField, FormItem, FormMessage } from "@/components/ui/form";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useToast } from "@/hooks/use-toast";

// Define validation schema for form fields
const loginSchema = z.object({
  email: z.string().email("Invalid email address"),
  password: z.string().min(6, "Password must be at least 6 characters"),
});

export default function SignInPage() {
  const navigate = useNavigate();
  const { toast } = useToast();  // Initialize toast

  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);

  // Initialize react-hook-form
  const form = useForm<z.infer<typeof loginSchema>>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const handleSignIn = async (values: z.infer<typeof loginSchema>) => {
    setLoading(true);
    setError(null);

    try {
      const tools = await signIn(values.email, values.password);
      console.log("User Tools: ", tools);

      // Success toast
      toast({
        title: "Success!",
        description: "You are now logged in",
      });

      localStorage.setItem("authToken", `${values.email}`)

      // Redirect user after successful login
      navigate("/dashboard"); 
    } catch (error) {
      // Ensure error is displayed properly
      const errorMessage = error instanceof Error ? error.message : String(error);

      // Show error toast
      toast({
        title: "Error!",
        description: errorMessage, // Display actual backend error
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <motion.div 
      className="flex flex-col items-center justify-center h-screen gap-6 bg-background text-foreground" 
      initial={{ opacity: 0, y: -30 }} 
      animate={{ opacity: 1, y: 0 }} 
      transition={{ duration: 0.5 }}
    >
      <Card className="w-full max-w-md shadow-md">
        <CardHeader>
          <CardTitle className="text-center text-2xl font-bold">Sign In</CardTitle>
        </CardHeader>

        <CardContent>
          {error && <p className="text-red-500 text-sm text-center">{error}</p>}

          <Form {...form}>
            <form onSubmit={form.handleSubmit(handleSignIn)} className="space-y-4">
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <Label>Email</Label>
                    <FormControl>
                      <Input type="email" placeholder="Enter your email" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <Label>Password</Label>
                    <FormControl>
                      <Input type="password" placeholder="Enter your password" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Signing In..." : "Sign In"}
              </Button>
            </form>
          </Form>
        </CardContent>

        <CardFooter className="text-sm text-center">
          {/* Back Button */}
          <Button 
            variant="outline" 
            className="w-full mt-4 border border-border text-foreground hover:bg-muted"
            onClick={() => navigate(-1)}
          >
            Go Back
          </Button>
        </CardFooter>
      </Card>
    </motion.div>
  );
}
