#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Ceph service name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<String>,
}

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
    #[doc = "Start ceph services."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
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
    #[doc = "Start ceph services."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
