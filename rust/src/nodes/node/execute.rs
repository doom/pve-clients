#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "JSON encoded array of commands."]
    pub commands: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct ExecuteClient<T> {
    client: T,
    path: String,
}

impl<T> ExecuteClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "execute"),
        }
    }
}
impl<T> ExecuteClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute multiple commands in order."]
    pub fn post(&self, parameters: PostParameters) -> Result<Vec<PostResponseItem>, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncExecuteClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncExecuteClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "execute"),
        }
    }
}
impl<T> AsyncExecuteClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute multiple commands in order."]
    pub async fn post(
        &self,
        parameters: PostParameters,
    ) -> Result<Vec<PostResponseItem>, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
