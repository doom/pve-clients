#[derive(Debug, Clone)]
pub struct CrushClient<T> {
    client: T,
    path: String,
}

impl<T> CrushClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "crush"),
        }
    }
}
impl<T> CrushClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get OSD crush map"]
    pub fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCrushClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCrushClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "crush"),
        }
    }
}
impl<T> AsyncCrushClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get OSD crush map"]
    pub async fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
