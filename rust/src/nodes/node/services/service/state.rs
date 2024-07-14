#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct StateClient<T> {
    client: T,
    path: String,
}

impl<T> StateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "state"),
        }
    }
}
impl<T> StateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read service properties"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "state"),
        }
    }
}
impl<T> AsyncStateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read service properties"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
