#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "base64-path to the directory or file to download."]
    pub filepath: String,
    #[doc = "Backup volume ID or name. Currently only PBS snapshots are supported."]
    pub volume: String,
}

#[derive(Debug, Clone)]
pub struct DownloadClient<T> {
    client: T,
    path: String,
}

impl<T> DownloadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "download"),
        }
    }
}
impl<T> DownloadClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Extract a file or directory (as zip archive) from a PBS backup."]
    pub fn get(&self, parameters: GetParameters) -> Result<serde_json::Value, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDownloadClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDownloadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "download"),
        }
    }
}
impl<T> AsyncDownloadClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Extract a file or directory (as zip archive) from a PBS backup."]
    pub async fn get(&self, parameters: GetParameters) -> Result<serde_json::Value, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
