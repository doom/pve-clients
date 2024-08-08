#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<u64>,
    #[doc = "Delete the original disk after successful copy. By default the original disk is kept as unused disk."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub delete: Option<bool>,
    #[doc = "Prevent changes if current configuration file has different SHA1\" 		    .\" digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "The disk you want to move."]
    pub disk: String,
    #[doc = "Target Format."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,
    #[doc = "Target storage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Prevent changes if the current config file of the target VM has a\" 		    .\" different SHA1 digest. This can be used to detect concurrent modifications."]
    #[serde(
        rename = "target-digest",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_digest: Option<String>,
    #[doc = "The config key the disk will be moved to on the target VM (for example, ide0 or scsi1). Default is the source disk key."]
    #[serde(
        rename = "target-disk",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_disk: Option<String>,
    #[doc = "The (unique) ID of the VM."]
    #[serde(
        rename = "target-vmid",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_vmid: Option<u64>,
}

impl PostParameters {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            bwlimit: Default::default(),
            delete: Default::default(),
            digest: Default::default(),
            format: Default::default(),
            storage: Default::default(),
            target_digest: Default::default(),
            target_disk: Default::default(),
            target_vmid: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveDiskClient<T> {
    client: T,
    path: String,
}

impl<T> MoveDiskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "move_disk"),
        }
    }
}
impl<T> MoveDiskClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Move volume to different storage or to a different VM."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMoveDiskClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMoveDiskClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "move_disk"),
        }
    }
}
impl<T> AsyncMoveDiskClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Move volume to different storage or to a different VM."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
