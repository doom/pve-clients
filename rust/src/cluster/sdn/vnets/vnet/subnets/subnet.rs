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
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
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
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
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
}

#[derive(Debug, Clone)]
pub struct SubnetClient<T> {
    client: T,
    path: String,
}

impl<T> SubnetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, subnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, subnet),
        }
    }
}
impl<T> SubnetClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete sdn subnet object configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read sdn subnet configuration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Update sdn subnet object configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSubnetClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSubnetClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, subnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, subnet),
        }
    }
}
impl<T> AsyncSubnetClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete sdn subnet object configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read sdn subnet configuration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Update sdn subnet object configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
