#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "The key (qemu monitor encoding)."]
    pub key: String,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct SendkeyClient<T> {
    client: T,
    path: String,
}

impl<T> SendkeyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sendkey"),
        }
    }
}
impl<T> SendkeyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Send key event to virtual machine."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSendkeyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSendkeyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sendkey"),
        }
    }
}
impl<T> AsyncSendkeyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Send key event to virtual machine."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
