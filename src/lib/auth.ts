// When using the Tauri API:
import { invoke } from '@tauri-apps/api/core';

export async function initialSignUp(email: string, password: string, orgName: string, userName: string) {
    try {
        await invoke("initial_sign_up", { email, password, orgName, userName});
        console.log("User successfully signed up");
    } catch (error) {
        console.error("Sign-up error:", error);

        // Ensure error is formatted as a string
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}

export async function inviteUser(orgId: string, email: string) {
    try {
        const inviteCode = await invoke("invite_user", { orgId, email });
        console.log("Invite sent successfully:", inviteCode);
    } catch (error) {
        console.error("Invite error:", error);

        // Ensure error is formatted correctly
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}

export async function inviteSignUp(email: string, password: string, invite_code: string, user_name: string) {
    try {
        const inviteCode = await invoke("invite_sign_up", { email, password, invite_code, user_name });
        console.log("Invite sent successfully:", inviteCode);
    } catch (error) {
        console.error("Invite error:", error);

        // Ensure error is formatted correctly
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}

export async function signIn(email: string, password: string): Promise<string[]> {
    try {
        const tools: string[] = await invoke("sign_in", { email, password });
        console.log("User signed in successfully. Tools allowed:", tools);
        return tools;
    } catch (error) {
        console.error("Sign-in error:", error);
        
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}
