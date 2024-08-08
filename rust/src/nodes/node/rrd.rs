#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The RRD consolidation function"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cf: Option<String>,
    #[doc = "The list of datasources you want to display."]
    pub ds: String,
    #[doc = "Specify the time frame you are interested in."]
    pub timeframe: String,
}

impl GetParameters {
    pub fn new(ds: String, timeframe: String) -> Self {
        Self {
            ds,
            timeframe,
            cf: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub filename: String,
}

#[derive(Debug, Clone)]
pub struct RrdClient<T> {
    client: T,
    path: String,
}

impl<T> RrdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rrd"),
        }
    }
}
impl<T> RrdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read node RRD statistics (returns PNG)"]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRrdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRrdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rrd"),
        }
    }
}
impl<T> AsyncRrdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read node RRD statistics (returns PNG)"]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
