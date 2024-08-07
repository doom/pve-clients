pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub cidr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: String,
    pub name: String,
}

impl GetResponseItem {
    pub fn new(cidr: String, digest: String, name: String) -> Self {
        Self {
            cidr,
            digest,
            name,
            comment: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Network/IP specification in CIDR format."]
    pub cidr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Alias name."]
    pub name: String,
}

impl PostParameters {
    pub fn new(cidr: String, name: String) -> Self {
        Self {
            cidr,
            name,
            comment: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AliasesClient<T> {
    client: T,
    path: String,
}

impl<T> AliasesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "aliases"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AliasesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List aliases"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create IP or Network Alias."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAliasesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAliasesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "aliases"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncAliasesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List aliases"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create IP or Network Alias."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
