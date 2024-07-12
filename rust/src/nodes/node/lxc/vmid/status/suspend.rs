#[derive(Debug, Clone)]
pub struct SuspendClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend"),
        }
    }
}
impl<T> SuspendClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Suspend the container. This is experimental."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend"),
        }
    }
}
impl<T> AsyncSuspendClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Suspend the container. This is experimental."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
