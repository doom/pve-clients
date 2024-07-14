pub mod iface;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list specific interface types."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub address: Option<String>,
    #[doc = "IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub address6: Option<String>,
    #[doc = "Automatically start interface on boot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub autostart: Option<bool>,
    #[doc = "Specify the primary interface for active-backup bond."]
    #[serde(
        rename = "bond-primary",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub bond_primary: Option<String>,
    #[doc = "Bonding mode."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bond_mode: Option<String>,
    #[doc = "Selects the transmit hash policy to use for slave selection in balance-xor and 802.3ad modes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bond_xmit_hash_policy: Option<String>,
    #[doc = "Specify the interfaces you want to add to your bridge."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bridge_ports: Option<String>,
    #[doc = "Enable bridge vlan support."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub bridge_vlan_aware: Option<bool>,
    #[doc = "IPv4 CIDR."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cidr: Option<String>,
    #[doc = "IPv6 CIDR."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cidr6: Option<String>,
    #[doc = "Comments"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comments: Option<String>,
    #[doc = "Comments"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comments6: Option<String>,
    #[doc = "Default gateway address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gateway: Option<String>,
    #[doc = "Default ipv6 gateway address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gateway6: Option<String>,
    #[doc = "Network interface name."]
    pub iface: String,
    #[doc = "MTU."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtu: Option<u64>,
    #[doc = "Network mask."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub netmask: Option<String>,
    #[doc = "Network mask."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub netmask6: Option<u64>,
    #[doc = "Specify the interfaces used by the bonding device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ovs_bonds: Option<String>,
    #[doc = "The OVS bridge associated with a OVS port. This is required when you create an OVS port."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ovs_bridge: Option<String>,
    #[doc = "OVS interface options."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ovs_options: Option<String>,
    #[doc = "Specify the interfaces you want to add to your bridge."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ovs_ports: Option<String>,
    #[doc = "Specify a VLan tag (used by OVSPort, OVSIntPort, OVSBond)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ovs_tag: Option<u64>,
    #[doc = "Specify the interfaces used by the bonding device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub slaves: Option<String>,
    #[doc = "Network interface type"]
    pub r#type: String,
    #[doc = "vlan-id for a custom named vlan interface (ifupdown2 only)."]
    #[serde(rename = "vlan-id", skip_serializing_if = "Option::is_none", default)]
    pub vlan_id: Option<u64>,
    #[doc = "Specify the raw interface for the vlan interface."]
    #[serde(
        rename = "vlan-raw-device",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub vlan_raw_device: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkClient<T> {
    client: T,
    path: String,
}

impl<T> NetworkClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "network"),
        }
    }

    pub fn iface(&self, iface: &str) -> iface::IfaceClient<T> {
        iface::IfaceClient::<T>::new(self.client.clone(), &self.path, iface)
    }
}
impl<T> NetworkClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Revert network configuration changes."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "List available networks"]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create network device configuration"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Reload network configuration"]
    pub fn put(&self) -> Result<String, T::Error> {
        self.client.put(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNetworkClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNetworkClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "network"),
        }
    }

    pub fn iface(&self, iface: &str) -> iface::AsyncIfaceClient<T> {
        iface::AsyncIfaceClient::<T>::new(self.client.clone(), &self.path, iface)
    }
}
impl<T> AsyncNetworkClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Revert network configuration changes."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "List available networks"]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create network device configuration"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Reload network configuration"]
    pub async fn put(&self) -> Result<String, T::Error> {
        self.client.put(&self.path, &()).await
    }
}
