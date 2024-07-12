#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Target node."]
    pub node: String,
}

#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}

impl<T> MigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Request resource migration (online) to another node."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMigrateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrate"),
        }
    }
}
impl<T> AsyncMigrateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Request resource migration (online) to another node."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
