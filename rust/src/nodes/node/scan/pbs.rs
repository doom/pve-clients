#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Certificate SHA 256 fingerprint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fingerprint: Option<String>,
    #[doc = "User password or API token secret."]
    pub password: String,
    #[doc = "Optional port."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "The server address (name or IP)."]
    pub server: String,
    #[doc = "User-name or API token-ID."]
    pub username: String,
}

impl GetParameters {
    pub fn new(password: String, server: String, username: String) -> Self {
        Self {
            password,
            server,
            username,
            fingerprint: Default::default(),
            port: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Comment from server."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "The datastore name."]
    pub store: String,
}

impl GetResponseItem {
    pub fn new(store: String) -> Self {
        Self {
            store,
            comment: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PbsClient<T> {
    client: T,
    path: String,
}

impl<T> PbsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pbs"),
        }
    }
}
impl<T> PbsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan remote Proxmox Backup Server."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPbsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPbsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pbs"),
        }
    }
}
impl<T> AsyncPbsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan remote Proxmox Backup Server."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
