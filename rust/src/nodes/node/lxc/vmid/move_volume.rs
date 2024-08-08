#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<f64>,
    #[doc = "Delete the original volume after successful copy. By default the original is kept as an unused volume entry."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub delete: Option<bool>,
    #[doc = "Prevent changes if current configuration file has different SHA1 \" . 		    \"digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Target Storage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Prevent changes if current configuration file of the target \" . 		    \"container has a different SHA1 digest. This can be used to prevent \" . 		    \"concurrent modifications."]
    #[serde(
        rename = "target-digest",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_digest: Option<String>,
    #[doc = "The (unique) ID of the VM."]
    #[serde(
        rename = "target-vmid",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_vmid: Option<u64>,
    #[doc = "The config key the volume will be moved to. Default is the source volume key."]
    #[serde(
        rename = "target-volume",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_volume: Option<String>,
    #[doc = "Volume which will be moved."]
    pub volume: String,
}

impl PostParameters {
    pub fn new(volume: String) -> Self {
        Self {
            volume,
            bwlimit: Default::default(),
            delete: Default::default(),
            digest: Default::default(),
            storage: Default::default(),
            target_digest: Default::default(),
            target_vmid: Default::default(),
            target_volume: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveVolumeClient<T> {
    client: T,
    path: String,
}

impl<T> MoveVolumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "move_volume"),
        }
    }
}
impl<T> MoveVolumeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Move a rootfs-/mp-volume to a different storage or to a different container."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMoveVolumeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMoveVolumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "move_volume"),
        }
    }
}
impl<T> AsyncMoveVolumeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Move a rootfs-/mp-volume to a different storage or to a different container."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
