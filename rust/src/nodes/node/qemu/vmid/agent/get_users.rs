#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetUsersClient<T> {
    client: T,
    path: String,
}

impl<T> GetUsersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-users"),
        }
    }
}
impl<T> GetUsersClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-users."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetUsersClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetUsersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-users"),
        }
    }
}
impl<T> AsyncGetUsersClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-users."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
