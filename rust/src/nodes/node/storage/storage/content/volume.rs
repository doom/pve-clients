#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "Time to wait for the task to finish. We return 'null' if the task finish within that time."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delay: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)"]
    pub format: String,
    #[doc = "Optional notes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    #[doc = "The Path"]
    pub path: String,
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
    pub used: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Target volume identifier"]
    pub target: String,
    #[doc = "Target node. Default is local node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "The new notes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notes: Option<String>,
    #[doc = "Protection status. Currently only supported for backups."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protected: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct VolumeClient<T> {
    client: T,
    path: String,
}

impl<T> VolumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, volume: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, volume),
        }
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete volume"]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<Option<String>, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Get volume attributes"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Copy a volume. This is experimental code - do not use."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update volume attributes"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVolumeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVolumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, volume: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, volume),
        }
    }
}
impl<T> AsyncVolumeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete volume"]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<Option<String>, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Get volume attributes"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Copy a volume. This is experimental code - do not use."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update volume attributes"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
