pub mod zone;

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
    #[doc = "Only list SDN zones of specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dhcp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dnszone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ipam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtu: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reversedns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    pub r#type: String,
    pub zone: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Advertise evpn subnets if you have silent hosts"]
    #[serde(
        rename = "advertise-subnets",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bridge: Option<String>,
    #[doc = "Disable auto mac learning."]
    #[serde(
        rename = "bridge-disable-mac-learning",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub bridge_disable_mac_learning: Option<bool>,
    #[doc = "Frr router name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub controller: Option<String>,
    #[doc = "Type of the DHCP backend for this zone"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dhcp: Option<String>,
    #[doc = "Disable ipv4 arp && ipv6 neighbour discovery suppression"]
    #[serde(
        rename = "disable-arp-nd-suppression",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable_arp_nd_suppression: Option<bool>,
    #[doc = "dns api server"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns: Option<String>,
    #[doc = "dns domain zone  ex: mydomain.com"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dnszone: Option<String>,
    #[doc = "Faucet dataplane id"]
    #[serde(rename = "dp-id", skip_serializing_if = "Option::is_none", default)]
    pub dp_id: Option<u64>,
    #[doc = "List of cluster node names."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exitnodes: Option<String>,
    #[doc = "Allow exitnodes to connect to evpn guests"]
    #[serde(
        rename = "exitnodes-local-routing",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub exitnodes_local_routing: Option<bool>,
    #[doc = "Force traffic to this exitnode first."]
    #[serde(
        rename = "exitnodes-primary",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exitnodes_primary: Option<String>,
    #[doc = "use a specific ipam"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ipam: Option<String>,
    #[doc = "Anycast logical router mac address"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mac: Option<String>,
    #[doc = "MTU"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtu: Option<u64>,
    #[doc = "List of cluster node names."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[doc = "peers address list."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub peers: Option<String>,
    #[doc = "reverse dns api server"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reversedns: Option<String>,
    #[doc = "Route-Target import"]
    #[serde(rename = "rt-import", skip_serializing_if = "Option::is_none", default)]
    pub rt_import: Option<String>,
    #[doc = "Service-VLAN Tag"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tag: Option<u64>,
    #[doc = "Plugin type."]
    pub r#type: String,
    #[serde(
        rename = "vlan-protocol",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub vlan_protocol: Option<String>,
    #[doc = "l3vni."]
    #[serde(rename = "vrf-vxlan", skip_serializing_if = "Option::is_none", default)]
    pub vrf_vxlan: Option<u64>,
    #[doc = "Vxlan tunnel udp port (default 4789)."]
    #[serde(
        rename = "vxlan-port",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub vxlan_port: Option<u64>,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
}

#[derive(Debug, Clone)]
pub struct ZonesClient<T> {
    client: T,
    path: String,
}

impl<T> ZonesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zones"),
        }
    }

    pub fn zone(&self, zone: &str) -> zone::ZoneClient<T> {
        zone::ZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN zones index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn zone object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncZonesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncZonesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zones"),
        }
    }

    pub fn zone(&self, zone: &str) -> zone::AsyncZoneClient<T> {
        zone::AsyncZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
impl<T> AsyncZonesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN zones index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn zone object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
