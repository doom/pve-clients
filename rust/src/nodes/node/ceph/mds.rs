pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub host: Option<String>,
    #[doc = "The name (ID) for the MDS"]
    pub name: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rank: Option<u64>,
    #[doc = "If true, the standby MDS is polling the active MDS for faster recovery (hot standby)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub standby_replay: Option<bool>,
    #[doc = "State of the MDS"]
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct MdsClient<T> {
    client: T,
    path: String,
}

impl<T> MdsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mds"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> MdsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "MDS directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMdsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMdsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mds"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncMdsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "MDS directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
