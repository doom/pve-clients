#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct SuspendRamClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendRamClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-ram"),
        }
    }
}
impl<T> SuspendRamClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute suspend-ram."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendRamClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendRamClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-ram"),
        }
    }
}
impl<T> AsyncSuspendRamClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute suspend-ram."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
