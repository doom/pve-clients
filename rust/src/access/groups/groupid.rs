#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GroupidClient<T> {
    client: T,
    path: String,
}

impl<T> GroupidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, groupid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, groupid),
        }
    }
}
impl<T> GroupidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete group."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get group configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update group data."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGroupidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGroupidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, groupid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, groupid),
        }
    }
}
impl<T> AsyncGroupidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete group."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get group configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update group data."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
