pub mod ips;
pub mod subnets;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Display pending config."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pending: Option<bool>,
    #[doc = "Display running config."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub running: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "alias name of the vnet"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub alias: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "vlan or vxlan id"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<u64>,
    #[doc = "Allow vm VLANs to pass through this vnet."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vlanaware: Option<bool>,
    #[doc = "zone id"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub zone: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VnetClient<T> {
    client: T,
    path: String,
}

impl<T> VnetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, vnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vnet),
        }
    }

    pub fn subnets(&self) -> subnets::SubnetsClient<T> {
        subnets::SubnetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ips(&self) -> ips::IpsClient<T> {
        ips::IpsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete sdn vnet object configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read sdn vnet configuration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Update sdn vnet object configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVnetClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVnetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, vnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vnet),
        }
    }

    pub fn subnets(&self) -> subnets::AsyncSubnetsClient<T> {
        subnets::AsyncSubnetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ips(&self) -> ips::AsyncIpsClient<T> {
        ips::AsyncIpsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncVnetClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete sdn vnet object configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read sdn vnet configuration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Update sdn vnet object configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
