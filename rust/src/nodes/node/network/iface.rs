#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub method: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
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
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Default gateway address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gateway: Option<String>,
    #[doc = "Default ipv6 gateway address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gateway6: Option<String>,
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

impl PutParameters {
    pub fn new(r#type: String) -> Self {
        Self {
            r#type,
            address: Default::default(),
            address6: Default::default(),
            autostart: Default::default(),
            bond_primary: Default::default(),
            bond_mode: Default::default(),
            bond_xmit_hash_policy: Default::default(),
            bridge_ports: Default::default(),
            bridge_vlan_aware: Default::default(),
            cidr: Default::default(),
            cidr6: Default::default(),
            comments: Default::default(),
            comments6: Default::default(),
            delete: Default::default(),
            gateway: Default::default(),
            gateway6: Default::default(),
            mtu: Default::default(),
            netmask: Default::default(),
            netmask6: Default::default(),
            ovs_bonds: Default::default(),
            ovs_bridge: Default::default(),
            ovs_options: Default::default(),
            ovs_ports: Default::default(),
            ovs_tag: Default::default(),
            slaves: Default::default(),
            vlan_id: Default::default(),
            vlan_raw_device: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfaceClient<T> {
    client: T,
    path: String,
}

impl<T> IfaceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, iface: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, iface),
        }
    }
}
impl<T> IfaceClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete network device configuration"]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read network device configuration"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update network device configuration"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIfaceClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIfaceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, iface: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, iface),
        }
    }
}
impl<T> AsyncIfaceClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete network device configuration"]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read network device configuration"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update network device configuration"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
