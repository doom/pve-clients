#[derive(Debug, Clone)]
pub struct TestClient<T> {
    client: T,
    path: String,
}

impl<T> TestClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "test"),
        }
    }
}
impl<T> TestClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Send a test notification to a provided target."]
    pub fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTestClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTestClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "test"),
        }
    }
}
impl<T> AsyncTestClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Send a test notification to a provided target."]
    pub async fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &()).await
    }
}
