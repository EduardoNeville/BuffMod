import { LoginForm } from "@/components/login-form"
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router";

export default function LoginPage() {
  const navigate = useNavigate();

  function handleClick() {
    navigate("/home");
  }

  return (

    <div className="flex min-h-svh flex-col items-center justify-center bg-muted p-6 md:p-10">
      <div className="w-full max-w-sm md:max-w-3xl">
        <LoginForm />
      </div>
      <Button type="button" onClick={handleClick}>
        Go home
      </Button>
    </div>
  )
}
