#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct FsfreezeFreezeClient<T> {
    client: T,
    path: String,
}

impl<T> FsfreezeFreezeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-freeze"),
        }
    }
}
impl<T> FsfreezeFreezeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute fsfreeze-freeze."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFsfreezeFreezeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFsfreezeFreezeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-freeze"),
        }
    }
}
impl<T> AsyncFsfreezeFreezeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute fsfreeze-freeze."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
