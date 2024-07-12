#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Whether the tasklog file should be downloaded. This parameter can't be used in conjunction with other parameters"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub download: Option<bool>,
    #[doc = "The amount of lines to read from the tasklog."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<u64>,
    #[doc = "Start at this line when reading the tasklog"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub start: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Line number"]
    pub n: u64,
    #[doc = "Line text"]
    pub t: String,
}

#[derive(Debug, Clone)]
pub struct LogClient<T> {
    client: T,
    path: String,
}

impl<T> LogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "log"),
        }
    }
}
impl<T> LogClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read task log."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLogClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "log"),
        }
    }
}
impl<T> AsyncLogClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read task log."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
