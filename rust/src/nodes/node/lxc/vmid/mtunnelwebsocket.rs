#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "unix socket to forward to"]
    pub socket: String,
    #[doc = "ticket return by initial 'mtunnel' API call, or retrieved via 'ticket' tunnel command"]
    pub ticket: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub socket: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MtunnelwebsocketClient<T> {
    client: T,
    path: String,
}

impl<T> MtunnelwebsocketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mtunnelwebsocket"),
        }
    }
}
impl<T> MtunnelwebsocketClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Migration tunnel endpoint for websocket upgrade - only for internal use by VM migration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMtunnelwebsocketClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMtunnelwebsocketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mtunnelwebsocket"),
        }
    }
}
impl<T> AsyncMtunnelwebsocketClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Migration tunnel endpoint for websocket upgrade - only for internal use by VM migration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
