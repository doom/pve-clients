#[derive(Debug, Clone)]
pub struct InClient<T> {
    client: T,
    path: String,
}

impl<T> InClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "in"),
        }
    }
}
impl<T> InClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ceph osd in"]
    pub fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "in"),
        }
    }
}
impl<T> AsyncInClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ceph osd in"]
    pub async fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &()).await
    }
}
