import { useNavigate } from "react-router";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { motion } from "framer-motion";

import { initialSignUp } from "@/lib/auth";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useToast } from "@/hooks/use-toast"; // Import Toast Hook

// Define Form Schema using Zod
const formSchema = z.object({
  username: z.string().max(20, {message: "UserName has to be at most 20 characters"}),
  email: z.string().email({ message: "Invalid email format." }),
  password: z.string().min(6, { message: "Password must be at least 6 characters." }),
  orgName: z.string().min(3, { message: "Organization name must be at least 3 characters." }),
});

export function CreateOrganizationPage() {
  const navigate = useNavigate();
  const { toast } = useToast();  // Initialize toast

  // Set up react-hook-form with Zod
  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
      orgName: "",
    },
  });

  const handleCreateOrganization = async (values: z.infer<typeof formSchema>) => {
    console.log("Sanitization works...");

    try {
      await initialSignUp(values.email, values.password, values.orgName, values.username);

      // Success toast
      toast({
        title: "Success!",
        description: "Organization and account created successfully.",
      });

      localStorage.setItem("authToken", `${values.email}-${values.orgName}`)

      // Navigate to dashboard
      navigate("/dashboard");
    } catch (error) {
      console.error("Signup Error:", error);

      // Ensure error is displayed properly
      const errorMessage = error instanceof Error ? error.message : String(error);

      // Show error toast
      toast({
        title: "Error!",
        description: errorMessage, // Display actual backend error
      });
    }
  };

  return (
    <motion.div 
      className="flex flex-col items-center justify-center h-screen gap-6 bg-background text-foreground" 
      initial={{ opacity: 0, y: -30 }} 
      animate={{ opacity: 1, y: 0 }} 
      transition={{ duration: 0.5 }}
    >
      <Card className="w-full max-w-md bg-card text-card-foreground border border-border shadow-md">
        <CardContent className="p-6">
          <CardHeader className="text-center text-2xl font-bold">
            Create Organization
          </CardHeader>
          <CardDescription>
            <Form {...form}>
              <form onSubmit={form.handleSubmit(handleCreateOrganization)} className="grid gap-4 mt-4">
                {/* UserName Field */}
                <FormField 
                  control={form.control} 
                  name="username" 
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel className="text-muted-foreground">User Name</FormLabel>
                      <FormControl>
                        <Input placeholder="username" {...field} className="bg-background border border-input focus:ring focus:ring-ring" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                
                {/* Email Field */}
                <FormField 
                  control={form.control} 
                  name="email" 
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel className="text-muted-foreground">Email</FormLabel>
                      <FormControl>
                        <Input placeholder="you@example.com" {...field} className="bg-background border border-input focus:ring focus:ring-ring" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Password Field */}
                <FormField 
                  control={form.control} 
                  name="password" 
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel className="text-muted-foreground">Password</FormLabel>
                      <FormControl>
                        <Input type="password" placeholder="******" {...field} className="bg-background border border-input focus:ring focus:ring-ring" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Organization Name Field */}
                <FormField 
                  control={form.control} 
                  name="orgName" 
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel className="text-muted-foreground">Organization Name</FormLabel>
                      <FormControl>
                        <Input placeholder="Your Organization" {...field} className="bg-background border border-input focus:ring focus:ring-ring" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Submit Button */}
                <Button type="submit" className="w-full bg-primary text-primary-foreground hover:bg-opacity-90">
                  Create
                </Button>

                {/* Back Button */}
                <Button 
                  variant="outline" 
                  className="w-full mt-4 border border-border text-foreground hover:bg-muted"
                  onClick={() => navigate(-1)}
                >
                  Go Back
                </Button>

              </form>
            </Form>
          </CardDescription>
        </CardContent>
      </Card>
    </motion.div>
  );
}
