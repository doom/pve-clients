#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The command as a list of program + arguments"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub command: Option<String>,
    #[doc = "Data to pass as 'input-data' to the guest. Usually treated as STDIN to 'command'."]
    #[serde(
        rename = "input-data",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub input_data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    #[doc = "The PID of the process started by the guest-agent."]
    pub pid: u64,
}

#[derive(Debug, Clone)]
pub struct ExecClient<T> {
    client: T,
    path: String,
}

impl<T> ExecClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "exec"),
        }
    }
}
impl<T> ExecClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Executes the given command in the vm via the guest-agent and returns an object with the pid."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncExecClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncExecClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "exec"),
        }
    }
}
impl<T> AsyncExecClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Executes the given command in the vm via the guest-agent and returns an object with the pid."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
