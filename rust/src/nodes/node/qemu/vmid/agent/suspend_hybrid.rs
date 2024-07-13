#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct SuspendHybridClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendHybridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-hybrid"),
        }
    }
}
impl<T> SuspendHybridClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute suspend-hybrid."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendHybridClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendHybridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-hybrid"),
        }
    }
}
impl<T> AsyncSuspendHybridClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute suspend-hybrid."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
