pub mod pos;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub pos: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Rule action ('ACCEPT', 'DROP', 'REJECT') or security group name."]
    pub action: String,
    #[doc = "Descriptive comment."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Restrict packet destination address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dest: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
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
    #[doc = "Update rule at position <pos>."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pos: Option<u64>,
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
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct RulesClient<T> {
    client: T,
    path: String,
}

impl<T> RulesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rules"),
        }
    }

    pub fn pos(&self, pos: &str) -> pos::PosClient<T> {
        pos::PosClient::<T>::new(self.client.clone(), &self.path, pos)
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List rules."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new rule."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRulesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRulesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rules"),
        }
    }

    pub fn pos(&self, pos: &str) -> pos::AsyncPosClient<T> {
        pos::AsyncPosClient::<T>::new(self.client.clone(), &self.path, pos)
    }
}
impl<T> AsyncRulesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List rules."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new rule."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
