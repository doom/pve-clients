#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct ShutdownClient<T> {
    client: T,
    path: String,
}

impl<T> ShutdownClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "shutdown"),
        }
    }
}
impl<T> ShutdownClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute shutdown."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncShutdownClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncShutdownClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "shutdown"),
        }
    }
}
impl<T> AsyncShutdownClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute shutdown."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
