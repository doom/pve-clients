#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Seconds since 1970-01-01 00:00:00 (local time)"]
    pub localtime: u64,
    #[doc = "Seconds since 1970-01-01 00:00:00 UTC."]
    pub time: u64,
    #[doc = "Time zone"]
    pub timezone: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Time zone. The file '/usr/share/zoneinfo/zone.tab' contains the list of valid names."]
    pub timezone: String,
}

#[derive(Debug, Clone)]
pub struct TimeClient<T> {
    client: T,
    path: String,
}

impl<T> TimeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "time"),
        }
    }
}
impl<T> TimeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read server time and time zone settings."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set time zone."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTimeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTimeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "time"),
        }
    }
}
impl<T> AsyncTimeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read server time and time zone settings."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set time zone."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
