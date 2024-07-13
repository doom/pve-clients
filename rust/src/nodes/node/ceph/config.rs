#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}

impl<T> ConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the Ceph configuration file. Deprecated, please use `/nodes/{node}/ceph/cfg/raw."]
    pub fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncConfigClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> AsyncConfigClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the Ceph configuration file. Deprecated, please use `/nodes/{node}/ceph/cfg/raw."]
    pub async fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
