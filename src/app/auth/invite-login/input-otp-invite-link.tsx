import {
  InputOTP,
  InputOTPGroup,
  InputOTPSeparator,
  InputOTPSlot,
} from "@/components/ui/input-otp";
import { useState } from "react";

export function InputOTPInviteLink({ onChange }: { onChange: (code: string) => void }) {
  const [otp, setOtp] = useState("");

  const handleInputChange = (newOtp: string) => {
    // Remove dashes from the input
    const cleanedOtp = newOtp.replace(/-/g, "").substring(0, 8);
    setOtp(cleanedOtp);
    onChange(cleanedOtp); // Send cleaned OTP to parent component
  };

  return (
    <div className="flex flex-col items-center gap-3 w-full">
      <p className="text-xs text-muted-foreground">
        Dashes (-) will be automatically removed when pasting your code.
      </p>

      <InputOTP
        maxLength={8}
        value={otp}
        onChange={handleInputChange}
        className="flex flex-col items-center gap-2 text-sm"
      >
        {/* First Group: 4 Characters */}
        <InputOTPGroup className="gap-1">
          {[...Array(4)].map((_, index) => (
            <InputOTPSlot key={index} index={index} />
          ))}
        </InputOTPGroup>
        <InputOTPSeparator />

        {/* Second Group: 4 Characters */}
        <InputOTPGroup className="gap-1">
          {[...Array(4)].map((_, index) => (
            <InputOTPSlot key={index + 4} index={index + 4} />
          ))}
        </InputOTPGroup>
      </InputOTP>
    </div>
  );
}
