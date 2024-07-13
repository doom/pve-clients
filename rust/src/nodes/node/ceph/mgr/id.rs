#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}

impl<T> IdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy Ceph Manager."]
    pub fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Create Ceph Manager"]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> AsyncIdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy Ceph Manager."]
    pub async fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Create Ceph Manager"]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
