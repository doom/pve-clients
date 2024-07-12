#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct ManagerStatusClient<T> {
    client: T,
    path: String,
}

impl<T> ManagerStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "manager_status"),
        }
    }
}
impl<T> ManagerStatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get full HA manger status, including LRM status."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncManagerStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncManagerStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "manager_status"),
        }
    }
}
impl<T> AsyncManagerStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get full HA manger status, including LRM status."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
