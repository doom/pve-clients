#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub start: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Line number"]
    pub n: u64,
    #[doc = "Line text"]
    pub t: String,
}

#[derive(Debug, Clone)]
pub struct LogClient<T> {
    client: T,
    path: String,
}

impl<T> LogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "log"),
        }
    }
}
impl<T> LogClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read replication job log."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLogClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "log"),
        }
    }
}
impl<T> AsyncLogClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read replication job log."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
