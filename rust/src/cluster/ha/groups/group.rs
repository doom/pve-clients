#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "List of cluster node names with optional priority."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[doc = "The CRM tries to run services on the node with the highest priority. If a node with higher priority comes online, the CRM migrates the service to that node. Enabling nofailback prevents that behavior."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nofailback: Option<bool>,
    #[doc = "Resources bound to restricted groups may only run on nodes defined by the group."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub restricted: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct GroupClient<T> {
    client: T,
    path: String,
}

impl<T> GroupClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, group: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, group),
        }
    }
}
impl<T> GroupClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete ha group configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read ha group configuration."]
    pub fn get(&self) -> Result<serde_json::Value, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update ha group configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGroupClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGroupClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, group: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, group),
        }
    }
}
impl<T> AsyncGroupClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete ha group configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read ha group configuration."]
    pub async fn get(&self) -> Result<serde_json::Value, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update ha group configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
