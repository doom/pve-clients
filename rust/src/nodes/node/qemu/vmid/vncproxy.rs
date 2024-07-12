#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Generates a random password to be used as ticket instead of the API ticket."]
    #[serde(
        rename = "generate-password",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub generate_password: Option<bool>,
    #[doc = "Prepare for websocket upgrade (only required when using serial terminal, otherwise upgrade is always possible)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub websocket: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub cert: String,
    #[doc = "Returned if requested with 'generate-password' param. Consists of printable ASCII characters ('!' .. '~')."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    pub port: u64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct VncproxyClient<T> {
    client: T,
    path: String,
}

impl<T> VncproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncproxy"),
        }
    }
}
impl<T> VncproxyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Creates a TCP VNC proxy connections."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVncproxyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVncproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncproxy"),
        }
    }
}
impl<T> AsyncVncproxyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Creates a TCP VNC proxy connections."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
