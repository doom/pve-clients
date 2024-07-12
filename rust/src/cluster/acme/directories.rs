#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub name: String,
    #[doc = "URL of ACME CA directory endpoint."]
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct DirectoriesClient<T> {
    client: T,
    path: String,
}

impl<T> DirectoriesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "directories"),
        }
    }
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get named known ACME directory endpoints."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDirectoriesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDirectoriesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "directories"),
        }
    }
}
impl<T> AsyncDirectoriesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get named known ACME directory endpoints."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
