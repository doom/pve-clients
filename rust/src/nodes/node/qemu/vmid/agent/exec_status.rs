#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The PID to query"]
    pub pid: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "stderr of the process"]
    #[serde(rename = "err-data", skip_serializing_if = "Option::is_none", default)]
    pub err_data: Option<String>,
    #[doc = "true if stderr was not fully captured"]
    #[serde(
        rename = "err-truncated",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub err_truncated: Option<bool>,
    #[doc = "process exit code if it was normally terminated."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exitcode: Option<u64>,
    #[doc = "Tells if the given command has exited yet."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub exited: bool,
    #[doc = "stdout of the process"]
    #[serde(rename = "out-data", skip_serializing_if = "Option::is_none", default)]
    pub out_data: Option<String>,
    #[doc = "true if stdout was not fully captured"]
    #[serde(
        rename = "out-truncated",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub out_truncated: Option<bool>,
    #[doc = "signal number or exception code if the process was abnormally terminated."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signal: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ExecStatusClient<T> {
    client: T,
    path: String,
}

impl<T> ExecStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "exec-status"),
        }
    }
}
impl<T> ExecStatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Gets the status of the given pid started by the guest-agent"]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncExecStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncExecStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "exec-status"),
        }
    }
}
impl<T> AsyncExecStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Gets the status of the given pid started by the guest-agent"]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
