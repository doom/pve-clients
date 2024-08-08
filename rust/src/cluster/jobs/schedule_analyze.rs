#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Number of event-iteration to simulate and return."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iterations: Option<u64>,
    #[doc = "Job schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: String,
    #[doc = "UNIX timestamp to start the calculation from. Defaults to the current time."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub starttime: Option<u64>,
}

impl GetParameters {
    pub fn new(schedule: String) -> Self {
        Self {
            schedule,
            iterations: Default::default(),
            starttime: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "UNIX timestamp for the run."]
    pub timestamp: u64,
    #[doc = "UTC timestamp for the run."]
    pub utc: String,
}

#[derive(Debug, Clone)]
pub struct ScheduleAnalyzeClient<T> {
    client: T,
    path: String,
}

impl<T> ScheduleAnalyzeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "schedule-analyze"),
        }
    }
}
impl<T> ScheduleAnalyzeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a list of future schedule runtimes."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncScheduleAnalyzeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncScheduleAnalyzeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "schedule-analyze"),
        }
    }
}
impl<T> AsyncScheduleAnalyzeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a list of future schedule runtimes."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
