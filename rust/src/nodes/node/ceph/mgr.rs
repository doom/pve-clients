pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub host: Option<String>,
    #[doc = "The name (ID) for the MGR"]
    pub name: serde_json::Value,
    #[doc = "State of the MGR"]
    pub state: String,
}

impl GetResponseItem {
    pub fn new(name: serde_json::Value, state: String) -> Self {
        Self {
            name,
            state,
            addr: Default::default(),
            host: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MgrClient<T> {
    client: T,
    path: String,
}

impl<T> MgrClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mgr"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> MgrClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "MGR directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMgrClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMgrClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mgr"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncMgrClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "MGR directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
