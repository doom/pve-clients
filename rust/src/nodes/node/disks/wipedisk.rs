#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Block device name"]
    pub disk: String,
}

#[derive(Debug, Clone)]
pub struct WipediskClient<T> {
    client: T,
    path: String,
}

impl<T> WipediskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "wipedisk"),
        }
    }
}
impl<T> WipediskClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Wipe a disk or partition."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncWipediskClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncWipediskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "wipedisk"),
        }
    }
}
impl<T> AsyncWipediskClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Wipe a disk or partition."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
