pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub alloc: u64,
    pub dedup: f64,
    pub frag: u64,
    pub free: u64,
    pub health: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Configure storage using the zpool."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub add_storage: Option<bool>,
    #[doc = "Pool sector size exponent."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ashift: Option<u64>,
    #[doc = "The compression algorithm to use."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub compression: Option<String>,
    #[doc = "The block devices you want to create the zpool on."]
    pub devices: String,
    #[serde(
        rename = "draid-config",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub draid_config: Option<String>,
    #[doc = "The storage identifier."]
    pub name: String,
    #[doc = "The RAID level to use."]
    pub raidlevel: String,
}

impl PostParameters {
    pub fn new(devices: String, name: String, raidlevel: String) -> Self {
        Self {
            devices,
            name,
            raidlevel,
            add_storage: Default::default(),
            ashift: Default::default(),
            compression: Default::default(),
            draid_config: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZfsClient<T> {
    client: T,
    path: String,
}

impl<T> ZfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zfs"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List Zpools."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a ZFS pool."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncZfsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncZfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zfs"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncZfsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List Zpools."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a ZFS pool."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
