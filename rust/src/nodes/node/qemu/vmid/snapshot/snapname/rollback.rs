#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Whether the VM should get started after rolling back successfully. (Note: VMs will be automatically started if the snapshot includes RAM.)"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub start: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct RollbackClient<T> {
    client: T,
    path: String,
}

impl<T> RollbackClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rollback"),
        }
    }
}
impl<T> RollbackClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Rollback VM state to specified snapshot."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRollbackClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRollbackClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rollback"),
        }
    }
}
impl<T> AsyncRollbackClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Rollback VM state to specified snapshot."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
