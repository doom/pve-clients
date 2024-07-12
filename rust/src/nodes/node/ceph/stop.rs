#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Ceph service name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<String>,
}

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
    #[doc = "Stop ceph services."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
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
    #[doc = "Stop ceph services."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
