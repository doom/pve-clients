#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The server address (name or IP)."]
    pub server: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "NFS export options."]
    pub options: String,
    #[doc = "The exported path."]
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct NfsClient<T> {
    client: T,
    path: String,
}

impl<T> NfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nfs"),
        }
    }
}
impl<T> NfsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan remote NFS server."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNfsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nfs"),
        }
    }
}
impl<T> AsyncNfsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan remote NFS server."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
