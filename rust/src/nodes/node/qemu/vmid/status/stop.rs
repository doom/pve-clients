#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Do not deactivate storage volumes."]
    #[serde(
        rename = "keepActive",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub keep_active: Option<bool>,
    #[doc = "The cluster node name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migratedfrom: Option<String>,
    #[doc = "Try to abort active 'qmshutdown' tasks before stopping."]
    #[serde(
        rename = "overrule-shutdown",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub overrule_shutdown: Option<bool>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
    #[doc = "Wait maximal timeout seconds."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct StopClient<T> {
    client: T,
    path: String,
}

impl<T> StopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> StopClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Stop virtual machine. The qemu process will exit immediately. This is akin to pulling the power plug of a running computer and may damage the VM data."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStopClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> AsyncStopClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Stop virtual machine. The qemu process will exit immediately. This is akin to pulling the power plug of a running computer and may damage the VM data."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
