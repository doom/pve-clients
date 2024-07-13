#[derive(Debug, Clone)]
pub struct ScheduleNowClient<T> {
    client: T,
    path: String,
}

impl<T> ScheduleNowClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "schedule_now"),
        }
    }
}
impl<T> ScheduleNowClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Schedule replication job to start as soon as possible."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncScheduleNowClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncScheduleNowClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "schedule_now"),
        }
    }
}
impl<T> AsyncScheduleNowClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Schedule replication job to start as soon as possible."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
