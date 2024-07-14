pub mod volume;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list content of this type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[doc = "Only list images for this VM"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Creation time (seconds since the UNIX Epoch)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ctime: Option<u64>,
    #[doc = "If whole backup is encrypted, value is the fingerprint or '1'  if encrypted. Only useful for the Proxmox Backup Server storage type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub encrypted: Option<String>,
    #[doc = "Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)"]
    pub format: String,
    #[doc = "Optional notes. If they contain multiple lines, only the first one is returned here."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    #[doc = "Volume identifier of parent (for linked cloned)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<String>,
    #[doc = "Protection status. Currently only supported for backups."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protected: Option<bool>,
    #[doc = "Volume size in bytes."]
    pub size: u64,
    #[doc = "Used space. Please note that most storage plugins do not report anything useful here."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub used: Option<u64>,
    #[doc = "Last backup verification result, only useful for PBS storages."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verification: Option<Verification>,
    #[doc = "Associated Owner VMID."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
    #[doc = "Volume identifier."]
    pub volid: String,
}

#[doc = "Last backup verification result, only useful for PBS storages."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Verification {
    #[doc = "Last backup verification state."]
    pub state: String,
    #[doc = "Last backup verification UPID."]
    pub upid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The name of the file to create."]
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,
    #[doc = "Size in kilobyte (1024 bytes). Optional suffixes 'M' (megabyte, 1024K) and 'G' (gigabyte, 1024M)"]
    pub size: String,
    #[doc = "Specify owner VM"]
    pub vmid: u64,
}

#[derive(Debug, Clone)]
pub struct ContentClient<T> {
    client: T,
    path: String,
}

impl<T> ContentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "content"),
        }
    }

    pub fn volume(&self, volume: &str) -> volume::VolumeClient<T> {
        volume::VolumeClient::<T>::new(self.client.clone(), &self.path, volume)
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List storage content."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Allocate disk images."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncContentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncContentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "content"),
        }
    }

    pub fn volume(&self, volume: &str) -> volume::AsyncVolumeClient<T> {
        volume::AsyncVolumeClient::<T>::new(self.client.clone(), &self.path, volume)
    }
}
impl<T> AsyncContentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List storage content."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Allocate disk images."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
