#[derive(Debug, Clone)]
pub struct CurrentClient<T> {
    client: T,
    path: String,
}

impl<T> CurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> CurrentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get HA manger status."]
    pub fn get(&self) -> Result<Vec<serde_json::Value>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCurrentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> AsyncCurrentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get HA manger status."]
    pub async fn get(&self) -> Result<Vec<serde_json::Value>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
