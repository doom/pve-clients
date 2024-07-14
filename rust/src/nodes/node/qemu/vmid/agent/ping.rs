#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct PingClient<T> {
    client: T,
    path: String,
}

impl<T> PingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ping"),
        }
    }
}
impl<T> PingClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute ping."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPingClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ping"),
        }
    }
}
impl<T> AsyncPingClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute ping."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
