#[derive(Debug, Clone)]
pub struct ReloadClient<T> {
    client: T,
    path: String,
}

impl<T> ReloadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "reload"),
        }
    }
}
impl<T> ReloadClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Reload service. Falls back to restart if service cannot be reloaded."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncReloadClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncReloadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "reload"),
        }
    }
}
impl<T> AsyncReloadClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Reload service. Falls back to restart if service cannot be reloaded."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
