#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
    #[doc = "The storage for the VM state"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statestorage: Option<String>,
    #[doc = "If set, suspends the VM to disk. Will be resumed on next VM start."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub todisk: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct SuspendClient<T> {
    client: T,
    path: String,
}

impl<T> SuspendClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend"),
        }
    }
}
impl<T> SuspendClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Suspend virtual machine."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSuspendClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSuspendClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "suspend"),
        }
    }
}
impl<T> AsyncSuspendClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Suspend virtual machine."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
