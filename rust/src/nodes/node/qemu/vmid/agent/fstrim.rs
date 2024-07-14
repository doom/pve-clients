#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct FstrimClient<T> {
    client: T,
    path: String,
}

impl<T> FstrimClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fstrim"),
        }
    }
}
impl<T> FstrimClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute fstrim."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFstrimClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFstrimClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "fstrim"),
        }
    }
}
impl<T> AsyncFstrimClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute fstrim."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
