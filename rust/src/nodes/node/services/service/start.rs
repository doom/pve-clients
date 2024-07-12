#[derive(Debug, Clone)]
pub struct StartClient<T> {
    client: T,
    path: String,
}

impl<T> StartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "start"),
        }
    }
}
impl<T> StartClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Start service."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStartClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "start"),
        }
    }
}
impl<T> AsyncStartClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Start service."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
