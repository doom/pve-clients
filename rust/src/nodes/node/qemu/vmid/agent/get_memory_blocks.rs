#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetMemoryBlocksClient<T> {
    client: T,
    path: String,
}

impl<T> GetMemoryBlocksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-memory-blocks"),
        }
    }
}
impl<T> GetMemoryBlocksClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-memory-blocks."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetMemoryBlocksClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetMemoryBlocksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-memory-blocks"),
        }
    }
}
impl<T> AsyncGetMemoryBlocksClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-memory-blocks."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
