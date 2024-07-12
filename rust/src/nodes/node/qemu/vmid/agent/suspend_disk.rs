#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct SuspendDiskClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendDiskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-disk"),
        }
    }
}
impl<T> SuspendDiskClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute suspend-disk."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendDiskClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendDiskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend-disk"),
        }
    }
}
impl<T> AsyncSuspendDiskClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute suspend-disk."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
