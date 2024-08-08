pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The mounted device."]
    pub device: String,
    #[doc = "The mount options."]
    pub options: String,
    #[doc = "The mount path."]
    pub path: String,
    #[doc = "The filesystem type."]
    pub r#type: String,
    #[doc = "The path of the mount unit."]
    pub unitfile: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Configure storage using the directory."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the filesystem on."]
    pub device: String,
    #[doc = "The desired filesystem."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filesystem: Option<String>,
    #[doc = "The storage identifier."]
    pub name: String,
}

impl PostParameters {
    pub fn new(device: String, name: String) -> Self {
        Self {
            device,
            name,
            add_storage: Default::default(),
            filesystem: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryClient<T> {
    client: T,
    path: String,
}

impl<T> DirectoryClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "directory"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> DirectoryClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "PVE Managed Directory storages."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDirectoryClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDirectoryClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "directory"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncDirectoryClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "PVE Managed Directory storages."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
