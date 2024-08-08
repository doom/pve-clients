pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The name of the thinpool."]
    pub lv: String,
    #[doc = "The size of the thinpool in bytes."]
    pub lv_size: u64,
    #[doc = "The size of the metadata lv in bytes."]
    pub metadata_size: u64,
    #[doc = "The used bytes of the metadata lv."]
    pub metadata_used: u64,
    #[doc = "The used bytes of the thinpool."]
    pub used: u64,
    #[doc = "The associated volume group."]
    pub vg: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Configure storage using the thinpool."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the thinpool on."]
    pub device: String,
    #[doc = "The storage identifier."]
    pub name: String,
}

impl PostParameters {
    pub fn new(device: String, name: String) -> Self {
        Self {
            device,
            name,
            add_storage: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LvmthinClient<T> {
    client: T,
    path: String,
}

impl<T> LvmthinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvmthin"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> LvmthinClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List LVM thinpools"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create an LVM thinpool"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLvmthinClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLvmthinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvmthin"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncLvmthinClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List LVM thinpools"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create an LVM thinpool"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
