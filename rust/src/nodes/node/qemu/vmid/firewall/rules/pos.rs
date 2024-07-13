#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable: Option<u64>,
    #[serde(rename = "icmp-type", skip_serializing_if = "Option::is_none", default)]
    pub icmp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ipversion: Option<u64>,
    #[doc = "Log level for firewall rule"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#macro: Option<String>,
    pub pos: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sport: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Rule action ('ACCEPT', 'DROP', 'REJECT') or security group name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub action: Option<String>,
    #[doc = "Descriptive comment."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Restrict packet destination address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dest: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Restrict TCP/UDP destination port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dport: Option<String>,
    #[doc = "Flag to enable/disable a rule."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable: Option<u64>,
    #[doc = "Specify icmp-type. Only valid if proto equals 'icmp' or 'icmpv6'/'ipv6-icmp'."]
    #[serde(rename = "icmp-type", skip_serializing_if = "Option::is_none", default)]
    pub icmp_type: Option<String>,
    #[doc = "Network interface name. You have to use network configuration key names for VMs and containers ('net\\d+'). Host related rules can use arbitrary strings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iface: Option<String>,
    #[doc = "Log level for firewall rule."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log: Option<String>,
    #[doc = "Use predefined standard macro."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#macro: Option<String>,
    #[doc = "Move rule to new position <moveto>. Other arguments are ignored."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub moveto: Option<u64>,
    #[doc = "IP protocol. You can use protocol names ('tcp'/'udp') or simple numbers, as defined in '/etc/protocols'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proto: Option<String>,
    #[doc = "Restrict packet source address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[doc = "Restrict TCP/UDP source port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sport: Option<String>,
    #[doc = "Rule type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PosClient<T> {
    client: T,
    path: String,
}

impl<T> PosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, pos: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pos),
        }
    }
}
impl<T> PosClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete rule."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Get single rule data."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Modify rule data."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPosClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, pos: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pos),
        }
    }
}
impl<T> AsyncPosClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete rule."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Get single rule data."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Modify rule data."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
