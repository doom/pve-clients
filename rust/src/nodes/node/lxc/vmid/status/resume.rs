#[derive(Debug, Clone)]
pub struct ResumeClient<T> {
    client: T,
    path: String,
}

impl<T> ResumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resume"),
        }
    }
}
impl<T> ResumeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Resume the container."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncResumeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncResumeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resume"),
        }
    }
}
impl<T> AsyncResumeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Resume the container."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
