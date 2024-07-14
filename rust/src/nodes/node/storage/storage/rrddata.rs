#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The RRD consolidation function"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cf: Option<String>,
    #[doc = "Specify the time frame you are interested in."]
    pub timeframe: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct RrddataClient<T> {
    client: T,
    path: String,
}

impl<T> RrddataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rrddata"),
        }
    }
}
impl<T> RrddataClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read storage RRD statistics."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRrddataClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRrddataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rrddata"),
        }
    }
}
impl<T> AsyncRrddataClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read storage RRD statistics."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
