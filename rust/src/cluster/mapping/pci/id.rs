#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Description of the logical PCI device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "A list of maps for the cluster nodes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub map: Option<Vec<String>>,
    #[doc = "Marks the device(s) as being capable of providing mediated devices."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mdev: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}

impl<T> IdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove Hardware Mapping."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get PCI Mapping."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update a hardware mapping."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> AsyncIdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove Hardware Mapping."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get PCI Mapping."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update a hardware mapping."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
