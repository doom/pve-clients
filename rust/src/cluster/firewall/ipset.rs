pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    pub digest: String,
    #[doc = "IP set name."]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "IP set name."]
    pub name: String,
    #[doc = "Rename an existing IPSet. You can set 'rename' to the same value as 'name' to update the 'comment' of an existing IPSet."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rename: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IpsetClient<T> {
    client: T,
    path: String,
}

impl<T> IpsetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ipset"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> IpsetClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List IPSets"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new IPSet"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIpsetClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIpsetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ipset"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncIpsetClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List IPSets"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new IPSet"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
