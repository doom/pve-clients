#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "With webauthn the format of half-authenticated tickts changed. New clients should pass 1 here and not worry about the old format. The old format is deprecated and will be retired with PVE-8.0"]
    #[serde(
        rename = "new-format",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub new_format: Option<bool>,
    #[doc = "One-time password for Two-factor authentication."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub otp: Option<String>,
    #[doc = "The secret password. This can also be a valid ticket."]
    pub password: String,
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
    #[doc = "You can optionally pass the realm using this parameter. Normally the realm is simply added to the username <username>@<relam>."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub realm: Option<String>,
    #[doc = "The signed TFA challenge string the user wants to respond to."]
    #[serde(
        rename = "tfa-challenge",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub tfa_challenge: Option<String>,
    #[doc = "User name"]
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    #[serde(
        rename = "CSRFPreventionToken",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub csrfprevention_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub clustername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ticket: Option<String>,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct TicketClient<T> {
    client: T,
    path: String,
}

impl<T> TicketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ticket"),
        }
    }
}
impl<T> TicketClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Dummy. Useful for formatters which want to provide a login page."]
    pub fn get(&self) -> Result<(), T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create or verify authentication ticket."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTicketClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTicketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ticket"),
        }
    }
}
impl<T> AsyncTicketClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Dummy. Useful for formatters which want to provide a login page."]
    pub async fn get(&self) -> Result<(), T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create or verify authentication ticket."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
