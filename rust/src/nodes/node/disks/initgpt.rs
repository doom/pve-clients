#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Block device name"]
    pub disk: String,
    #[doc = "UUID for the GPT table"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uuid: Option<String>,
}

impl PostParameters {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            uuid: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InitgptClient<T> {
    client: T,
    path: String,
}

impl<T> InitgptClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "initgpt"),
        }
    }
}
impl<T> InitgptClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Initialize Disk with GPT"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInitgptClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInitgptClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "initgpt"),
        }
    }
}
impl<T> AsyncInitgptClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Initialize Disk with GPT"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
