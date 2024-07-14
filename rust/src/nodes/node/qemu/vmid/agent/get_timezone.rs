#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetTimezoneClient<T> {
    client: T,
    path: String,
}

impl<T> GetTimezoneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-timezone"),
        }
    }
}
impl<T> GetTimezoneClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-timezone."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetTimezoneClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetTimezoneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-timezone"),
        }
    }
}
impl<T> AsyncGetTimezoneClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-timezone."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
