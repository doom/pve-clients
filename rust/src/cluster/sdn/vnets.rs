pub mod vnet;

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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "alias name of the vnet"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub alias: Option<String>,
    #[doc = "vlan or vxlan id"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<u64>,
    #[doc = "Type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
    #[doc = "Allow vm VLANs to pass through this vnet."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vlanaware: Option<bool>,
    #[doc = "The SDN vnet object identifier."]
    pub vnet: String,
    #[doc = "zone id"]
    pub zone: String,
}

#[derive(Debug, Clone)]
pub struct VnetsClient<T> {
    client: T,
    path: String,
}

impl<T> VnetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vnets"),
        }
    }

    pub fn vnet(&self, vnet: &str) -> vnet::VnetClient<T> {
        vnet::VnetClient::<T>::new(self.client.clone(), &self.path, vnet)
    }
}
impl<T> VnetsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN vnets index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn vnet object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVnetsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVnetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vnets"),
        }
    }

    pub fn vnet(&self, vnet: &str) -> vnet::AsyncVnetClient<T> {
        vnet::AsyncVnetClient::<T>::new(self.client.clone(), &self.path, vnet)
    }
}
impl<T> AsyncVnetsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN vnets index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn vnet object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
