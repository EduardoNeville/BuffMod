// When using the Tauri API:
import { invoke } from '@tauri-apps/api/core';
import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';
// when using `"withGlobalTauri": true`, you may use
// const { Client, Stronghold } = window.__TAURI__.stronghold;
import { appDataDir } from '@tauri-apps/api/path';
// when using `"withGlobalTauri": true`, you may use
// const { appDataDir } = window.__TAURI__.path;
import { sha256 } from 'js-sha256';

type Entry = {
  key: string,
  value: string
}

export async function initialSignUp(email: string, password: string, orgName: string, userName: string) {
    try {
        const entries: Entry[] = await invoke("initial_sign_up", { email, password, orgName, userName});
        await store_keys(entries);
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
        const entries: Entry[] = await invoke("invite_sign_up", { email, password, invite_code, user_name });
        await store_keys(entries);
    } catch (error) {
        console.error("Invite error:", error);

        // Ensure error is formatted correctly
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}

export async function signIn(email: string, password: string) {
    try {
        const entries: Entry[] = await invoke("sign_in", { email, password });
        await store_keys(entries);
    } catch (error) {
        console.error("Sign-in error:", error);
        const errorMessage = typeof error === "string" ? error : JSON.stringify(error);
        throw new Error(errorMessage);
    }
}

const store_keys = async (entries: Entry[]) => {
    // Pass on the user_id check sign_in in auth.rs
    const { stronghold, client } = await initStronghold(entries[2].value);
    const store = client.getStore();
    entries.map((e) => {
      insertRecord(store, e.key, e.value)
    });
    await stronghold.save();
};

const initStronghold = async (user_id: string) => {
  const vaultPath = `${await appDataDir()}/vault.hold`;
  const vaultPassword = sha256(user_id);
  const stronghold = await Stronghold.load(vaultPath, vaultPassword);

  let client: Client;
  const clientName = user_id;
  try {
    client = await stronghold.loadClient(clientName);
  } catch {
    client = await stronghold.createClient(clientName);
  }

  return {
    stronghold,
    client,
  };
};

// Insert a record to the store
async function insertRecord(store: any, key: string, value: string) {
  const data = Array.from(new TextEncoder().encode(value));
  await store.insert(key, data);
}

// Read a record from store
async function getRecord(store: any, key: string): Promise<string> {
  const data = await store.get(key);
  return new TextDecoder().decode(new Uint8Array(data));
}
