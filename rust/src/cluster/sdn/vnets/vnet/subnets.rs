pub mod subnet;

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
    #[doc = "IP address for the DNS server"]
    #[serde(
        rename = "dhcp-dns-server",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub dhcp_dns_server: Option<String>,
    #[doc = "A list of DHCP ranges for this subnet"]
    #[serde(
        rename = "dhcp-range",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub dhcp_range: Option<Vec<String>>,
    #[doc = "dns domain zone prefix  ex: 'adm' -> <hostname>.adm.mydomain.com"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dnszoneprefix: Option<String>,
    #[doc = "Subnet Gateway: Will be assign on vnet for layer3 zones"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gateway: Option<String>,
    #[doc = "enable masquerade for this subnet if pve-firewall"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub snat: Option<bool>,
    #[doc = "The SDN subnet object identifier."]
    pub subnet: String,
    pub r#type: String,
}

impl PostParameters {
    pub fn new(subnet: String, r#type: String) -> Self {
        Self {
            subnet,
            r#type,
            dhcp_dns_server: Default::default(),
            dhcp_range: Default::default(),
            dnszoneprefix: Default::default(),
            gateway: Default::default(),
            snat: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubnetsClient<T> {
    client: T,
    path: String,
}

impl<T> SubnetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "subnets"),
        }
    }

    pub fn subnet(&self, subnet: &str) -> subnet::SubnetClient<T> {
        subnet::SubnetClient::<T>::new(self.client.clone(), &self.path, subnet)
    }
}
impl<T> SubnetsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN subnets index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn subnet object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSubnetsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSubnetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "subnets"),
        }
    }

    pub fn subnet(&self, subnet: &str) -> subnet::AsyncSubnetClient<T> {
        subnet::AsyncSubnetClient::<T>::new(self.client.clone(), &self.path, subnet)
    }
}
impl<T> AsyncSubnetsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN subnets index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn subnet object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
