pub mod groupid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub groupid: String,
    #[doc = "list of users which form this group"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub users: Option<String>,
}

impl GetResponseItem {
    pub fn new(groupid: String) -> Self {
        Self {
            groupid,
            comment: Default::default(),
            users: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub groupid: String,
}

impl PostParameters {
    pub fn new(groupid: String) -> Self {
        Self {
            groupid,
            comment: Default::default(),
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

    pub fn groupid(&self, groupid: &str) -> groupid::GroupidClient<T> {
        groupid::GroupidClient::<T>::new(self.client.clone(), &self.path, groupid)
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Group index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new group."]
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

    pub fn groupid(&self, groupid: &str) -> groupid::AsyncGroupidClient<T> {
        groupid::AsyncGroupidClient::<T>::new(self.client.clone(), &self.path, groupid)
    }
}
impl<T> AsyncGroupsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Group index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new group."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
