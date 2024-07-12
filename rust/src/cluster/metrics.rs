pub mod server;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct MetricsClient<T> {
    client: T,
    path: String,
}

impl<T> MetricsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metrics"),
        }
    }

    pub fn server(&self) -> server::ServerClient<T> {
        server::ServerClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> MetricsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Metrics index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMetricsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMetricsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metrics"),
        }
    }

    pub fn server(&self) -> server::AsyncServerClient<T> {
        server::AsyncServerClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncMetricsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Metrics index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
