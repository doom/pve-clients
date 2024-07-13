#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The server address (name or IP)."]
    pub server: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The volume name."]
    pub volname: String,
}

#[derive(Debug, Clone)]
pub struct GlusterfsClient<T> {
    client: T,
    path: String,
}

impl<T> GlusterfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "glusterfs"),
        }
    }
}
impl<T> GlusterfsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan remote GlusterFS server."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGlusterfsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGlusterfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "glusterfs"),
        }
    }
}
impl<T> AsyncGlusterfsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan remote GlusterFS server."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
