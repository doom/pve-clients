pub mod realm_sync;
pub mod schedule_analyze;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "API sub-directory endpoint"]
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct JobsClient<T> {
    client: T,
    path: String,
}

impl<T> JobsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "jobs"),
        }
    }

    pub fn realm_sync(&self) -> realm_sync::RealmSyncClient<T> {
        realm_sync::RealmSyncClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn schedule_analyze(&self) -> schedule_analyze::ScheduleAnalyzeClient<T> {
        schedule_analyze::ScheduleAnalyzeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> JobsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index for jobs related endpoints."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncJobsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncJobsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "jobs"),
        }
    }

    pub fn realm_sync(&self) -> realm_sync::AsyncRealmSyncClient<T> {
        realm_sync::AsyncRealmSyncClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn schedule_analyze(&self) -> schedule_analyze::AsyncScheduleAnalyzeClient<T> {
        schedule_analyze::AsyncScheduleAnalyzeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncJobsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index for jobs related endpoints."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
