#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Volume identifier"]
    pub volume: String,
}

#[derive(Debug, Clone)]
pub struct ExtractconfigClient<T> {
    client: T,
    path: String,
}

impl<T> ExtractconfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "extractconfig"),
        }
    }
}
impl<T> ExtractconfigClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Extract configuration from vzdump backup archive."]
    pub fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncExtractconfigClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncExtractconfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "extractconfig"),
        }
    }
}
impl<T> AsyncExtractconfigClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Extract configuration from vzdump backup archive."]
    pub async fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
