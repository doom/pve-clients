#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub can_update_at_runtime: bool,
    pub level: String,
    pub mask: String,
    pub name: String,
    pub section: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct DbClient<T> {
    client: T,
    path: String,
}

impl<T> DbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "db"),
        }
    }
}
impl<T> DbClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the Ceph configuration database."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDbClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "db"),
        }
    }
}
impl<T> AsyncDbClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the Ceph configuration database."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
