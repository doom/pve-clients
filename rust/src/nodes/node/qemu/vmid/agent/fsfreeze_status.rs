#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct FsfreezeStatusClient<T> {
    client: T,
    path: String,
}

impl<T> FsfreezeStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-status"),
        }
    }
}
impl<T> FsfreezeStatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute fsfreeze-status."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFsfreezeStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFsfreezeStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-status"),
        }
    }
}
impl<T> AsyncFsfreezeStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute fsfreeze-status."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
