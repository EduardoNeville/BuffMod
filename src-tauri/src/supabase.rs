use reqwest::Client;
use serde_json::json;
use std::env::var;
use dotenv::dotenv;
use uuid::Uuid;

struct Keys {
    supabase_url: String,
    supabase_anon_key: String,
}

pub struct Supabase {
    client: Client,
    keys: Keys
}

impl Supabase {
    pub fn new() -> Self {
        dotenv().ok();
        Supabase {
            client: Client::new(),
            keys: Keys {
                supabase_url: var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
                supabase_anon_key: var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set"),
            }
        }
    }

    /// Initial sign-up process where a user creates an organization
    pub async fn initial_sign_up(&self, email: &str, password: &str, org_name: &str, user_name: &str) -> Result<(), String> {
        let res = self.client
            .post(format!("{}/auth/v1/signup", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await
            .map_err(|e| format!("Network error during sign-up: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Sign-up failed: {}", res.text().await.unwrap_or_else(|_| "Unknown error".to_string())));
        }

        let user_data = res.json::<serde_json::Value>().await
            .map_err(|e| format!("Error parsing sign-up response: {}", e))?;

        println!("User_data: {:?}", &user_data);

        if let Some(user_id) = user_data["id"].as_str() {
            let org_id = self.create_organization(user_id, org_name).await?;
            
            self.create_user_organization(user_id, &org_id, "admin", user_name).await?;
            self.grant_permissions(user_id, &org_id, true).await?;

            self.sign_in(email, password).await?;

            Ok(())
        } else {
            Err("Failed to retrieve user ID from sign-up response.".to_string())
        }
    }

    async fn create_organization(&self, user_id: &str, name: &str) -> Result<String, String> {
        // Step 1: Insert the new organization
        let res = self.client
            .post(format!("{}/rest/v1/organizations", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .header("Content-Type", "application/json")
            .json(&json!({
                "name": name,
                "owner_id": user_id
            }))
            .send()
            .await
            .map_err(|e| format!("Network error creating organization: {}", e))?;

        if !res.status().is_success() {
            return Err(format!(
                "Organization creation failed: {}",
                res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        // Step 2: Query the `organizations` table to find the newly inserted organization
        let org_query_res = self.client
            .get(format!(
                "{}/rest/v1/organizations?owner_id=eq.{}&name=eq.{}&select=id",
                &self.keys.supabase_url, user_id, name
            ))
            .header("apikey", &self.keys.supabase_anon_key)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error fetching organization: {}", e))?;

        if !org_query_res.status().is_success() {
            return Err(format!(
                "Failed to fetch created organization: {}",
                org_query_res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        // Step 3: Parse the response and extract the organization ID
        let org_data = org_query_res.json::<Vec<serde_json::Value>>().await
            .map_err(|e| format!("Error parsing organization retrieval response: {}", e))?;

        if let Some(org) = org_data.first() {
            if let Some(org_id) = org["id"].as_str() {
                Ok(org_id.to_string())
            } else {
                Err("Failed to retrieve organization ID.".to_string())
        }
    } else {
        Err("No organization found with the provided name and owner_id.".to_string())
    }
    }

    pub async fn create_invite(&self, organization_id: &str, email: &str) -> Result<String, String> {
        let invite_code = Uuid::new_v4().to_string();
        let res = self.client
            .post(format!("{}/rest/v1/invites", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .json(&json!({
                "organization_id": organization_id,
                "email": email,
                "invite_code": invite_code
            }))
            .send()
            .await
            .map_err(|e| format!("Network error creating invite: {}", e))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Invite creation failed: {}", error_text))
        } else {
            Ok(invite_code)
        }
    }

    async fn create_user_organization(&self, user_id: &str, org_id: &str, role: &str, user_name: &str) -> Result<(), String> {
        let res = self.client
            .post(format!("{}/rest/v1/user_organizations", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .json(&json!({
                "user_id": user_id,
                "organization_id": org_id,
                "role": role,
                "user_name": user_name
            }))
            .send()
            .await
            .map_err(|e| format!("Network error creating user organization: {}", e))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("User organization creation failed: {}", res.text().await.unwrap_or_else(|_| "Unknown error".to_string())))
        }
    }

    pub async fn invite_sign_up(&self, email: &str, password: &str, invite_code: &str, user_name: &str) -> Result<(), String> {
        let res = self.client
            .get(format!("{}/rest/v1/invites?invite_code=eq.{}", &self.keys.supabase_url, invite_code))
            .header("apikey", &self.keys.supabase_anon_key)
            .send()
            .await
            .map_err(|e| format!("Network error checking invite: {}", e))?;

        if !res.status().is_success() {
            return Err("Invalid invite code.".to_string());
        }

        let invites = res.json::<Vec<serde_json::Value>>().await.map_err(|e| format!("Error parsing invite checking response: {}", e))?;
        if let Some(invite) = invites.first() {
            if let Some(org_id) = invite["organization_id"].as_str() {
                
                let user_res = self.client
                    .post(format!("{}/auth/v1/signup", &self.keys.supabase_url))
                    .header("apikey", &self.keys.supabase_anon_key)
                    .json(&json!({
                        "email": email,
                        "password": password
                    }))
                    .send()
                    .await
                    .map_err(|e| format!("Network error during invite sign-up: {}", e))?;

                if user_res.status().is_success() {
                    let user_data = user_res.json::<serde_json::Value>().await
                        .map_err(|e| format!("Error parsing invite sign-up response: {}", e))?;

                    if let Some(user_id) = user_data["id"].as_str() {
                        self.create_user_organization(user_id, org_id, "staff", user_name).await?;
                        self.grant_permissions(user_id, org_id, false).await?;
                        return Ok(());
                    }
                }
            }
        }
        Err("Failed to process invite sign-up.".to_string())
    }

    async fn grant_permissions(&self, user_id: &str, org_id: &str, is_admin: bool) -> Result<(), String> {
        let tools = ["clients", "financials", "social-media", "permissions"];
        let mut permissions = Vec::new();

        for &tool in &tools {
            permissions.push(json!({
                "user_id": user_id,
                "organization_id": org_id,
                "tool_name": tool,
                "can_access": if tool == "permissions" { is_admin } else { true }
            }));
        }

        let res = self.client
            .post(format!("{}/rest/v1/permissions", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .json(&permissions)
            .send()
            .await
            .map_err(|e| format!("Network error setting permissions: {}", e))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err("Failed to grant permissions.".to_string())
        }
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<Vec<String>, String> {
        let res = self.client
            .post(format!("{}/auth/v1/token?grant_type=password", &self.keys.supabase_url))
            .header("apikey", &self.keys.supabase_anon_key)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await
            .map_err(|e| format!("Network error during sign-in: {}", e))?;

        if !res.status().is_success() {
            return Err(format!(
                "Sign-in failed: {}",
                res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        let user_data = res.json::<serde_json::Value>().await
            .map_err(|e| format!("Error parsing sign-in response: {}", e))?;

        println!("User_data: {:?}", &user_data);

        if let Some(user_id) = user_data["user"]["id"].as_str() {
            let tools = self.get_user_tools(user_id).await?;
            Ok(tools)
        } else {
            Err("Failed to retrieve user ID from sign-in response.".to_string())
        }
    }

    async fn get_user_tools(&self, user_id: &str) -> Result<Vec<String>, String> {
        let res = self.client
            .get(format!(
                "{}/rest/v1/permissions?user_id=eq.{}&can_access=eq.true&select=tool_name",
                &self.keys.supabase_url, user_id
            ))
            .header("apikey", &self.keys.supabase_anon_key)
            .send()
            .await
            .map_err(|e| format!("Network error fetching user tools: {}", e))?;

        if !res.status().is_success() {
            return Err(format!(
                "Failed to retrieve user tools: {}",
                res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        let permissions = res.json::<Vec<serde_json::Value>>().await
            .map_err(|e| format!("Error parsing tools response: {}", e))?;

        let tools = permissions.iter()
            .filter_map(|perm| perm["tool_name"].as_str().map(String::from))
            .collect();

        Ok(tools)
    }
}
