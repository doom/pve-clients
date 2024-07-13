#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetMemoryBlockInfoClient<T> {
    client: T,
    path: String,
}

impl<T> GetMemoryBlockInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-memory-block-info"),
        }
    }
}
impl<T> GetMemoryBlockInfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-memory-block-info."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetMemoryBlockInfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetMemoryBlockInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-memory-block-info"),
        }
    }
}
impl<T> AsyncGetMemoryBlockInfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-memory-block-info."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
