#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Issue start command even if virtual guest have 'onboot' not set or set to off."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Only consider guests from this comma separated list of VMIDs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StartallClient<T> {
    client: T,
    path: String,
}

impl<T> StartallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "startall"),
        }
    }
}
impl<T> StartallClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Start all VMs and containers located on this node (by default only those with onboot=1)."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStartallClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStartallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "startall"),
        }
    }
}
impl<T> AsyncStartallClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Start all VMs and containers located on this node (by default only those with onboot=1)."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
