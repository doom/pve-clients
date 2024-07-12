pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub children: Vec<GetResponseChildrenItem>,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub leaf: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseChildrenItem {
    #[doc = "The underlying physical volumes"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub children: Option<Vec<GetResponseChildrenItemChildrenItem>>,
    #[doc = "The free bytes in the volume group"]
    pub free: u64,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub leaf: bool,
    #[doc = "The name of the volume group"]
    pub name: String,
    #[doc = "The size of the volume group in bytes"]
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseChildrenItemChildrenItem {
    #[doc = "The free bytes in the physical volume"]
    pub free: u64,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub leaf: bool,
    #[doc = "The name of the physical volume"]
    pub name: String,
    #[doc = "The size of the physical volume in bytes"]
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Configure storage using the Volume Group"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the volume group on"]
    pub device: String,
    #[doc = "The storage identifier."]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LvmClient<T> {
    client: T,
    path: String,
}

impl<T> LvmClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvm"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> LvmClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List LVM Volume Groups"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create an LVM Volume Group"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLvmClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLvmClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvm"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncLvmClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List LVM Volume Groups"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create an LVM Volume Group"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
