pub mod monid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ceph_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ceph_version_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub direxists: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub host: Option<bool>,
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub quorum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rank: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
}

impl GetResponseItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            addr: Default::default(),
            ceph_version: Default::default(),
            ceph_version_short: Default::default(),
            direxists: Default::default(),
            host: Default::default(),
            quorum: Default::default(),
            rank: Default::default(),
            service: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonClient<T> {
    client: T,
    path: String,
}

impl<T> MonClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mon"),
        }
    }

    pub fn monid(&self, monid: &str) -> monid::MonidClient<T> {
        monid::MonidClient::<T>::new(self.client.clone(), &self.path, monid)
    }
}
impl<T> MonClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get Ceph monitor list."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMonClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMonClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mon"),
        }
    }

    pub fn monid(&self, monid: &str) -> monid::AsyncMonidClient<T> {
        monid::AsyncMonidClient::<T>::new(self.client.clone(), &self.path, monid)
    }
}
impl<T> AsyncMonClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get Ceph monitor list."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
