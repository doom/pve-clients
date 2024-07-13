#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The monitor command."]
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct MonitorClient<T> {
    client: T,
    path: String,
}

impl<T> MonitorClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "monitor"),
        }
    }
}
impl<T> MonitorClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute QEMU monitor commands."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMonitorClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMonitorClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "monitor"),
        }
    }
}
impl<T> AsyncMonitorClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute QEMU monitor commands."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
