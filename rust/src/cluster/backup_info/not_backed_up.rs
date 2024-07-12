#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Name of the guest"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "Type of the guest."]
    pub r#type: String,
    #[doc = "VMID of the guest."]
    pub vmid: u64,
}

#[derive(Debug, Clone)]
pub struct NotBackedUpClient<T> {
    client: T,
    path: String,
}

impl<T> NotBackedUpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "not-backed-up"),
        }
    }
}
impl<T> NotBackedUpClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Shows all guests which are not covered by any backup job."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNotBackedUpClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNotBackedUpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "not-backed-up"),
        }
    }
}
impl<T> AsyncNotBackedUpClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Shows all guests which are not covered by any backup job."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
