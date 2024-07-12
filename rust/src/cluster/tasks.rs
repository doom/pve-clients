#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub upid: String,
}

#[derive(Debug, Clone)]
pub struct TasksClient<T> {
    client: T,
    path: String,
}

impl<T> TasksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tasks"),
        }
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List recent tasks (cluster wide)."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTasksClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTasksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tasks"),
        }
    }
}
impl<T> AsyncTasksClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List recent tasks (cluster wide)."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
