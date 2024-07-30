pub mod group;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: String,
    #[doc = "Security Group name."]
    pub group: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Security Group name."]
    pub group: String,
    #[doc = "Rename/update an existing security group. You can set 'rename' to the same value as 'name' to update the 'comment' of an existing group."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rename: Option<String>,
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
    #[doc = "List security groups."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new security group."]
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
    #[doc = "List security groups."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new security group."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
