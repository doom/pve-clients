#[derive(Debug, Clone)]
pub struct StopClient<T> {
    client: T,
    path: String,
}

impl<T> StopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> StopClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Stop service."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStopClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> AsyncStopClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Stop service."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
