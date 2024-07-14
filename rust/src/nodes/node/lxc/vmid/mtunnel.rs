#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "List of network bridges to check availability. Will be checked again for actually used bridges during migration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bridges: Option<String>,
    #[doc = "List of storages to check permission and availability. Will be checked again for all actually used storages during migration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storages: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub socket: String,
    pub ticket: String,
    pub upid: String,
}

#[derive(Debug, Clone)]
pub struct MtunnelClient<T> {
    client: T,
    path: String,
}

impl<T> MtunnelClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mtunnel"),
        }
    }
}
impl<T> MtunnelClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Migration tunnel endpoint - only for internal use by CT migration."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMtunnelClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMtunnelClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mtunnel"),
        }
    }
}
impl<T> AsyncMtunnelClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Migration tunnel endpoint - only for internal use by CT migration."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
