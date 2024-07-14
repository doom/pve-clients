#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<u64>,
    #[doc = "Service ID"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<String>,
    #[doc = "Display all log since this date-time string."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub start: Option<u64>,
    #[doc = "Display all log until this date-time string."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub until: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Line number"]
    pub n: u64,
    #[doc = "Line text"]
    pub t: String,
}

#[derive(Debug, Clone)]
pub struct SyslogClient<T> {
    client: T,
    path: String,
}

impl<T> SyslogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "syslog"),
        }
    }
}
impl<T> SyslogClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read system log"]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSyslogClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSyslogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "syslog"),
        }
    }
}
impl<T> AsyncSyslogClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read system log"]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
