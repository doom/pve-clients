#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "SMB domain (Workgroup)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    #[doc = "User password."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "The server address (name or IP)."]
    pub server: String,
    #[doc = "User name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
}

impl GetParameters {
    pub fn new(server: String) -> Self {
        Self {
            server,
            domain: Default::default(),
            password: Default::default(),
            username: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Descriptive text from server."]
    pub description: String,
    #[doc = "The cifs share name."]
    pub share: String,
}

#[derive(Debug, Clone)]
pub struct CifsClient<T> {
    client: T,
    path: String,
}

impl<T> CifsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cifs"),
        }
    }
}
impl<T> CifsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan remote CIFS server."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCifsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCifsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cifs"),
        }
    }
}
impl<T> AsyncCifsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan remote CIFS server."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
