use dotenv::dotenv;
use reqwest::{Client, Error as ReqwestError, Response};
use serde::Serialize;
use serde_json::{json, Value};
use std::env::var;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SupabaseError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid response format")]
    ResponseFormatError,

    #[error("Supabase returned an error: {0}")]
    SupabaseError(String),

    #[error("Environment variable missing: {0}")]
    EnvVarMissing(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Invite code not found or invalid.")]
    InvalidInviteCode,

    #[error("Failed to create user in Supabase.")]
    UserCreationFailed,

    #[error("Failed to associate user with the organization.")]
    OrganizationAssociationFailed,

    #[error("Failed to grant permissions for the user.")]
    PermissionGrantFailed,

    #[error("Sign-in error: {0}")]
    SignInFailed(String),
}

impl Serialize for SupabaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Structure for Supabase API interactions
#[derive(Debug)]
pub struct Supabase {
    client: Client,
    supabase_url: String,
    supabase_anon_key: String,
}

impl Supabase {
    /// Create a new Supabase instance, handling environment variable errors
    pub fn new() -> Result<Self, SupabaseError> {
        dotenv().ok(); // Load environment variables

        let supabase_url =
            var("SUPABASE_URL").map_err(|_| SupabaseError::EnvVarMissing("SUPABASE_URL".into()))?;
        let supabase_anon_key = var("SUPABASE_ANON_KEY")
            .map_err(|_| SupabaseError::EnvVarMissing("SUPABASE_ANON_KEY".into()))?;

        Ok(Supabase {
            client: Client::new(),
            supabase_url,
            supabase_anon_key,
        })
    }

    /// Handles API response and extracts JSON
    async fn handle_response(response: Response) -> Result<Value, SupabaseError> {
        if response.status().is_success() {
            response
                .json::<Value>()
                .await
                .map_err(SupabaseError::NetworkError)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(SupabaseError::SupabaseError(error_text))
        }
    }

    /// Initial sign-up process where a user creates an organization
    pub async fn initial_sign_up(
        &self,
        email: &str,
        password: &str,
        org_name: &str,
        user_name: &str,
    ) -> Result<Value, SupabaseError> {
        let response = self
            .client
            .post(format!("{}/auth/v1/signup", &self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await?;

        let user_data = Supabase::handle_response(response).await?; // Get user data
        let user_id = user_data["id"]
            .as_str()
            .ok_or(SupabaseError::ResponseFormatError)?;

        // Create organization
        let org_id = self.create_organization(user_id, org_name).await?;

        // Create user organization role
        self.create_user_organization(user_id, &org_id, "admin", user_name)
            .await?;

        // Grant permissions
        self.grant_permissions(user_id, &org_id, true).await?;

        // Sign in again to retrieve updated tokens
        self.sign_in(email, password).await
    }

    /// Create a new organization and return its ID
    async fn create_organization(
        &self,
        user_id: &str,
        name: &str,
    ) -> Result<String, SupabaseError> {
        let response = self
            .client
            .post(format!("{}/rest/v1/organizations", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .header("Content-Type", "application/json")
            .json(&json!({
                "name": name,
                "owner_id": user_id
            }))
            .send()
            .await?;

        let org_data = Supabase::handle_response(response).await?;
        org_data["id"]
            .as_str()
            .map(String::from)
            .ok_or(SupabaseError::ResponseFormatError)
    }

    pub async fn create_an_invite(
        &self,
        organization_id: &str,
        email: &str,
    ) -> Result<String, SupabaseError> {
        let invite_code = Uuid::new_v4().to_string();

        // Step 1: Send request to create an invite
        let res = self.client
            .post(format!("{}/rest/v1/invites", &self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .json(&json!({
                "organization_id": organization_id,
                "email": email,
                "invite_code": invite_code
            }))
            .send()
            .await
            .map_err(SupabaseError::NetworkError)?;  

        // Step 2: Handle response failures properly
        if !res.status().is_success() {
            let error_text = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(SupabaseError::SupabaseError(format!(
                "Invite creation failed: {}", error_text
            )));
        }

        // Step 3: Return the invite code upon success
        Ok(invite_code)
    }

    /// Assign the user to the organization
    async fn create_user_organization(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
        user_name: &str,
    ) -> Result<(), SupabaseError> {
        let response = self
            .client
            .post(format!("{}/rest/v1/user_organizations", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .json(&json!({
                "user_id": user_id,
                "organization_id": org_id,
                "role": role,
                "user_name": user_name
            }))
            .send()
            .await?;

        Supabase::handle_response(response).await.map(|_| ())
    }

    pub async fn invite_sign_up(
        &self,
        email: &str,
        password: &str,
        invite_code: &str,
        user_name: &str,
    ) -> Result<Value, SupabaseError> {
        // Step 1: Retrieve the invite from Supabase
        let res = self.client
            .get(format!("{}/rest/v1/invites?invite_code=eq.{}", &self.supabase_url, invite_code))
            .header("apikey", &self.supabase_anon_key)
            .send()
            .await
            .map_err(SupabaseError::NetworkError)?;

        if !res.status().is_success() {
            return Err(SupabaseError::InvalidInviteCode);
        }

        let invites: Vec<Value> = res.json().await.map_err(|e| SupabaseError::ParsingError(e.to_string()))?;
        let invite = invites.get(0).ok_or(SupabaseError::InvalidInviteCode)?;
        
        let org_id = invite["organization_id"]
            .as_str()
            .ok_or(SupabaseError::ParsingError("Missing organization_id in invite".to_string()))?;

        // Step 2: Create a new user in Supabase
        let user_res = self.client
            .post(format!("{}/auth/v1/signup", &self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await
            .map_err(SupabaseError::NetworkError)?;

        if !user_res.status().is_success() {
            return Err(SupabaseError::UserCreationFailed);
        }

        let user_data: Value = user_res.json().await.map_err(|e| SupabaseError::ParsingError(e.to_string()))?;
        let user_id = user_data["id"]
            .as_str()
            .ok_or(SupabaseError::ParsingError("Missing user ID in response".to_string()))?;

        // Step 3: Associate the new user with the organization
        self.create_user_organization(user_id, org_id, "staff", user_name)
            .await
            .map_err(|_| SupabaseError::OrganizationAssociationFailed)?;

        // Step 4: Grant user permissions
        self.grant_permissions(user_id, org_id, false)
            .await
            .map_err(|_| SupabaseError::PermissionGrantFailed)?;

        // Step 5: Auto login user and return user data
        self.sign_in(email, password)
            .await
            .map_err(|e| SupabaseError::SignInFailed(e.to_string()))
    }

    /// Grant permissions to a user for specific tools
    async fn grant_permissions(
        &self,
        user_id: &str,
        org_id: &str,
        is_admin: bool,
    ) -> Result<(), SupabaseError> {
        // TODO: Query permissions and extract correct permission setup assigned by admin when
        // inviting
        let tools = ["clients", "financials", "social-media", "permissions"];
        let permissions: Vec<Value> = tools
            .iter()
            .map(|&tool| {
                json!({
                    "user_id": user_id,
                    "organization_id": org_id,
                    "tool_name": tool,
                    "can_access": if tool == "permissions" { is_admin } else { true }
                })
            })
            .collect();

        let response = self
            .client
            .post(format!("{}/rest/v1/permissions", self.supabase_url))
            .header("apikey", &self.supabase_anon_key)
            .json(&permissions)
            .send()
            .await?;

        Supabase::handle_response(response).await.map(|_| ())
    }

    /// Sign in and return authentication details (tokens, user_data)
    pub async fn sign_in(&self, email: &str, password: &str) -> Result<Value, SupabaseError> {
        let response = self
            .client
            .post(format!(
                "{}/auth/v1/token?grant_type=password",
                self.supabase_url
            ))
            .header("apikey", &self.supabase_anon_key)
            .json(&json!({
                "email": email,
                "password": password
            }))
            .send()
            .await?;

        Supabase::handle_response(response).await
    }

    async fn get_user_tools(&self, user_id: &str) -> Result<Vec<String>, String> {
        let res = self
            .client
            .get(format!(
                "{}/rest/v1/permissions?user_id=eq.{}&can_access=eq.true&select=tool_name",
                &self.supabase_url, user_id
            ))
            .header("apikey", &self.supabase_anon_key)
            .send()
            .await
            .map_err(|e| format!("Network error fetching user tools: {}", e))?;

        if !res.status().is_success() {
            return Err(format!(
                "Failed to retrieve user tools: {}",
                res.text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string())
            ));
        }

        let permissions = res
            .json::<Vec<serde_json::Value>>()
            .await
            .map_err(|e| format!("Error parsing tools response: {}", e))?;

        let tools = permissions
            .iter()
            .filter_map(|perm| perm["tool_name"].as_str().map(String::from))
            .collect();

        Ok(tools)
    }
}
