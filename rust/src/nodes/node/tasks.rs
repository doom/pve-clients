pub mod upid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list tasks with a status of ERROR."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub errors: Option<bool>,
    #[doc = "Only list this amount of tasks."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<u64>,
    #[doc = "Only list tasks since this UNIX epoch."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub since: Option<u64>,
    #[doc = "List archived, active or all tasks."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[doc = "List tasks beginning from this offset."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub start: Option<u64>,
    #[doc = "List of Task States that should be returned."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statusfilter: Option<String>,
    #[doc = "Only list tasks of this type (e.g., vzstart, vzdump)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub typefilter: Option<String>,
    #[doc = "Only list tasks until this UNIX epoch."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub until: Option<u64>,
    #[doc = "Only list tasks from this user."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub userfilter: Option<String>,
    #[doc = "Only list tasks for this VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub endtime: Option<u64>,
    pub id: String,
    pub node: String,
    pub pid: u64,
    pub pstart: u64,
    pub starttime: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    pub r#type: String,
    pub upid: String,
    pub user: String,
}

impl GetResponseItem {
    pub fn new(
        id: String,
        node: String,
        pid: u64,
        pstart: u64,
        starttime: u64,
        r#type: String,
        upid: String,
        user: String,
    ) -> Self {
        Self {
            id,
            node,
            pid,
            pstart,
            starttime,
            r#type,
            upid,
            user,
            endtime: Default::default(),
            status: Default::default(),
        }
    }
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

    pub fn upid(&self, upid: &str) -> upid::UpidClient<T> {
        upid::UpidClient::<T>::new(self.client.clone(), &self.path, upid)
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read task list for one node (finished tasks)."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
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

    pub fn upid(&self, upid: &str) -> upid::AsyncUpidClient<T> {
        upid::AsyncUpidClient::<T>::new(self.client.clone(), &self.path, upid)
    }
}
impl<T> AsyncTasksClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read task list for one node (finished tasks)."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
