#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Target node."]
    pub node: String,
}

#[derive(Debug, Clone)]
pub struct RelocateClient<T> {
    client: T,
    path: String,
}

impl<T> RelocateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "relocate"),
        }
    }
}
impl<T> RelocateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Request resource relocatzion to another node. This stops the service on the old node, and restarts it on the target node."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRelocateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRelocateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "relocate"),
        }
    }
}
impl<T> AsyncRelocateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Request resource relocatzion to another node. This stops the service on the old node, and restarts it on the target node."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
