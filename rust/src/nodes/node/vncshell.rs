#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Run specific command or default to login (requires 'root@pam')"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cmd: Option<String>,
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    #[serde(rename = "cmd-opts", skip_serializing_if = "Option::is_none", default)]
    pub cmd_opts: Option<String>,
    #[doc = "sets the height of the console in pixels."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub height: Option<u64>,
    #[doc = "use websocket instead of standard vnc."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub websocket: Option<bool>,
    #[doc = "sets the width of the console in pixels."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub width: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub cert: String,
    pub port: u64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct VncshellClient<T> {
    client: T,
    path: String,
}

impl<T> VncshellClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncshell"),
        }
    }
}
impl<T> VncshellClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Creates a VNC Shell proxy."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVncshellClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVncshellClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncshell"),
        }
    }
}
impl<T> AsyncVncshellClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Creates a VNC Shell proxy."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
