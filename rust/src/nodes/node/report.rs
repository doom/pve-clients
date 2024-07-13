#[derive(Debug, Clone)]
pub struct ReportClient<T> {
    client: T,
    path: String,
}

impl<T> ReportClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "report"),
        }
    }
}
impl<T> ReportClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Gather various systems information about a node"]
    pub fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncReportClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncReportClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "report"),
        }
    }
}
impl<T> AsyncReportClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Gather various systems information about a node"]
    pub async fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
