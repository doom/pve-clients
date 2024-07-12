#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetTimeClient<T> {
    client: T,
    path: String,
}

impl<T> GetTimeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-time"),
        }
    }
}
impl<T> GetTimeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-time."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetTimeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetTimeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-time"),
        }
    }
}
impl<T> AsyncGetTimeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-time."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
