#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Run specific command or default to login."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cmd: Option<String>,
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    #[serde(rename = "cmd-opts", skip_serializing_if = "Option::is_none", default)]
    pub cmd_opts: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub port: u64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct TermproxyClient<T> {
    client: T,
    path: String,
}

impl<T> TermproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "termproxy"),
        }
    }
}
impl<T> TermproxyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Creates a VNC Shell proxy."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTermproxyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTermproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "termproxy"),
        }
    }
}
impl<T> AsyncTermproxyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Creates a VNC Shell proxy."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
