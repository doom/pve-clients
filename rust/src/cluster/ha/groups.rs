pub mod group;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub group: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "The HA group identifier."]
    pub group: String,
    #[doc = "List of cluster node names with optional priority."]
    pub nodes: String,
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
    #[doc = "Group type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

impl PostParameters {
    pub fn new(group: String, nodes: String) -> Self {
        Self {
            group,
            nodes,
            comment: Default::default(),
            nofailback: Default::default(),
            restricted: Default::default(),
            r#type: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupsClient<T> {
    client: T,
    path: String,
}

impl<T> GroupsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "groups"),
        }
    }

    pub fn group(&self, group: &str) -> group::GroupClient<T> {
        group::GroupClient::<T>::new(self.client.clone(), &self.path, group)
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get HA groups."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new HA group."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGroupsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGroupsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "groups"),
        }
    }

    pub fn group(&self, group: &str) -> group::AsyncGroupClient<T> {
        group::AsyncGroupClient::<T>::new(self.client.clone(), &self.path, group)
    }
}
impl<T> AsyncGroupsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get HA groups."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new HA group."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
