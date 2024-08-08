#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exitstatus: Option<String>,
    pub id: String,
    pub node: String,
    pub pid: u64,
    pub starttime: f64,
    pub status: String,
    pub r#type: String,
    pub upid: String,
    pub user: String,
}

impl GetResponseItem {
    pub fn new(
        id: String,
        node: String,
        pid: u64,
        starttime: f64,
        status: String,
        r#type: String,
        upid: String,
        user: String,
    ) -> Self {
        Self {
            id,
            node,
            pid,
            starttime,
            status,
            r#type,
            upid,
            user,
            exitstatus: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}

impl<T> StatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read task status."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read task status."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
