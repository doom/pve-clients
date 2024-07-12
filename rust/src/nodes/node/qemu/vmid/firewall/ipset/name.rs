pub mod cidr;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    #[doc = "Delete all members of the IPSet, if there are any."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub cidr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    pub digest: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nomatch: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Network/IP specification in CIDR format."]
    pub cidr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nomatch: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }

    pub fn cidr(&self, cidr: &str) -> cidr::CidrClient<T> {
        cidr::CidrClient::<T>::new(self.client.clone(), &self.path, cidr)
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete IPSet"]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "List IPSet content"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Add IP or Network to IPSet."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }

    pub fn cidr(&self, cidr: &str) -> cidr::AsyncCidrClient<T> {
        cidr::AsyncCidrClient::<T>::new(self.client.clone(), &self.path, cidr)
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete IPSet"]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "List IPSet content"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Add IP or Network to IPSet."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
