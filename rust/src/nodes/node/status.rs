#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Specify the command."]
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}

impl<T> StatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read node status"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Reboot or shutdown a node."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read node status"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Reboot or shutdown a node."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
