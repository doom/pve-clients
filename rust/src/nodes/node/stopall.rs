#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Force a hard-stop after the timeout."]
    #[serde(
        rename = "force-stop",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force_stop: Option<bool>,
    #[doc = "Timeout for each guest shutdown task. Depending on `force-stop`, the shutdown gets then simply aborted or a hard-stop is forced."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
    #[doc = "Only consider Guests with these IDs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StopallClient<T> {
    client: T,
    path: String,
}

impl<T> StopallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stopall"),
        }
    }
}
impl<T> StopallClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Stop all VMs and Containers."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStopallClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStopallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stopall"),
        }
    }
}
impl<T> AsyncStopallClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Stop all VMs and Containers."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
