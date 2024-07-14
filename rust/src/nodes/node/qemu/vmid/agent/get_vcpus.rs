#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetVcpusClient<T> {
    client: T,
    path: String,
}

impl<T> GetVcpusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-vcpus"),
        }
    }
}
impl<T> GetVcpusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-vcpus."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetVcpusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetVcpusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-vcpus"),
        }
    }
}
impl<T> AsyncGetVcpusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-vcpus."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
