#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Config type."]
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct DumpClient<T> {
    client: T,
    path: String,
}

impl<T> DumpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "dump"),
        }
    }
}
impl<T> DumpClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get automatically generated cloudinit config."]
    pub fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDumpClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDumpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "dump"),
        }
    }
}
impl<T> AsyncDumpClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get automatically generated cloudinit config."]
    pub async fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
