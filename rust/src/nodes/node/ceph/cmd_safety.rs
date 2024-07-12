#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Action to check"]
    pub action: String,
    #[doc = "ID of the service"]
    pub id: String,
    #[doc = "Service type"]
    pub service: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "If it is safe to run the command."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub safe: bool,
    #[doc = "Status message given by Ceph."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CmdSafetyClient<T> {
    client: T,
    path: String,
}

impl<T> CmdSafetyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cmd-safety"),
        }
    }
}
impl<T> CmdSafetyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Heuristical check if it is safe to perform an action."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCmdSafetyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCmdSafetyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cmd-safety"),
        }
    }
}
impl<T> AsyncCmdSafetyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Heuristical check if it is safe to perform an action."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
