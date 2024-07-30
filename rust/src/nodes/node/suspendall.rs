#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Only consider Guests with these IDs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SuspendallClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspendall"),
        }
    }
}
impl<T> SuspendallClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Suspend all VMs."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendallClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspendall"),
        }
    }
}
impl<T> AsyncSuspendallClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Suspend all VMs."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
