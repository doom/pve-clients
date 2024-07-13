#[derive(Debug, Clone)]
pub struct RestartClient<T> {
    client: T,
    path: String,
}

impl<T> RestartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "restart"),
        }
    }
}
impl<T> RestartClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Hard restart service. Use reload if you want to reduce interruptions."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRestartClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRestartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "restart"),
        }
    }
}
impl<T> AsyncRestartClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Hard restart service. Use reload if you want to reduce interruptions."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
