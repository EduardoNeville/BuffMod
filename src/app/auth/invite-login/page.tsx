import { useNavigate } from "react-router";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { motion } from "framer-motion";

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

// Define form validation schema using Zod
const formSchema = z.object({
  inviteCode: z.string().regex(
    /^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$/,
    { message: "Invalid invite code format (must be UUID)." }
  ),
  email: z.string().email({ message: "Invalid email format." }),
  password: z.string().min(6, { message: "Password must be at least 6 characters." }),
});

export function InviteLoginPage() {
  const navigate = useNavigate();

  // ✅ Set up react-hook-form with Zod validation
  const form = useForm({
    resolver: zodResolver(formSchema),
    defaultValues: {
      inviteCode: "",
      email: "",
      password: "",
    },
  });

  const handleSubmit = async (values: z.infer<typeof formSchema>) => {
    console.log("Form submitted:", values);
    localStorage.setItem("authToken", "mock_token"); // Store authentication session
    navigate("/home"); // Redirect to dashboard
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
            Join Organization
          </CardHeader>
          <CardDescription>
            <Form {...form}>
              <form onSubmit={form.handleSubmit(handleSubmit)} className="grid gap-4 mt-4">
                
                {/* Invite Code Field */}
                <FormField 
                  control={form.control} 
                  name="inviteCode" 
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel className="text-muted-foreground">Invite Code</FormLabel>
                      <FormControl>
                        <Input 
                          placeholder="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
                          {...field}
                          className={`bg-background border ${form.formState.errors.inviteCode ? "border-destructive" : "border-input"} focus:ring focus:ring-ring`}
                        />
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
                        <Input 
                          placeholder="you@example.com"
                          {...field}
                          className={`bg-background border ${form.formState.errors.email ? "border-destructive" : "border-input"} focus:ring focus:ring-ring`}
                        />
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
                        <Input 
                          type="password"
                          placeholder="******"
                          {...field}
                          className={`bg-background border ${form.formState.errors.password ? "border-destructive" : "border-input"} focus:ring focus:ring-ring`}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {/* Submit Button */}
                <Button type="submit" className="w-full bg-primary text-primary-foreground hover:bg-opacity-90">
                  Join
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
