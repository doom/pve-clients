#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct FsfreezeThawClient<T> {
    client: T,
    path: String,
}

impl<T> FsfreezeThawClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-thaw"),
        }
    }
}
impl<T> FsfreezeThawClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute fsfreeze-thaw."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFsfreezeThawClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFsfreezeThawClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fsfreeze-thaw"),
        }
    }
}
impl<T> AsyncFsfreezeThawClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute fsfreeze-thaw."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
