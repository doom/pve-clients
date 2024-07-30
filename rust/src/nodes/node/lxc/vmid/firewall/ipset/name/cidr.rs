#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nomatch: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct CidrClient<T> {
    client: T,
    path: String,
}

impl<T> CidrClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, cidr: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, cidr),
        }
    }
}
impl<T> CidrClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove IP or Network from IPSet."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Read IP or Network settings from IPSet."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update IP or Network settings"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCidrClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCidrClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, cidr: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, cidr),
        }
    }
}
impl<T> AsyncCidrClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove IP or Network from IPSet."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Read IP or Network settings from IPSet."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update IP or Network settings"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
