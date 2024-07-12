#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Enable host firewall rules."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Log level for incoming traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_in: Option<String>,
    #[doc = "Log level for outgoing traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_out: Option<String>,
    #[doc = "Enable logging of conntrack information."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub log_nf_conntrack: Option<bool>,
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ndp: Option<bool>,
    #[doc = "Allow invalid packets on connection tracking."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nf_conntrack_allow_invalid: Option<bool>,
    #[doc = "Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_helpers: Option<String>,
    #[doc = "Maximum number of tracked connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_max: Option<u64>,
    #[doc = "Conntrack established timeout."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_tcp_timeout_established: Option<u64>,
    #[doc = "Conntrack syn recv timeout."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_tcp_timeout_syn_recv: Option<u64>,
    #[doc = "Enable SMURFS filter."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nosmurfs: Option<bool>,
    #[doc = "Enable synflood protection"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection_synflood: Option<bool>,
    #[doc = "Synflood protection rate burst by ip src."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protection_synflood_burst: Option<u64>,
    #[doc = "Synflood protection rate syn/sec by ip src."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protection_synflood_rate: Option<u64>,
    #[doc = "Log level for SMURFS filter."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smurf_log_level: Option<String>,
    #[doc = "Log level for illegal tcp flags filter."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tcp_flags_log_level: Option<String>,
    #[doc = "Filter illegal combinations of TCP flags."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tcpflags: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Enable host firewall rules."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Log level for incoming traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_in: Option<String>,
    #[doc = "Log level for outgoing traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_out: Option<String>,
    #[doc = "Enable logging of conntrack information."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub log_nf_conntrack: Option<bool>,
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ndp: Option<bool>,
    #[doc = "Allow invalid packets on connection tracking."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nf_conntrack_allow_invalid: Option<bool>,
    #[doc = "Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_helpers: Option<String>,
    #[doc = "Maximum number of tracked connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_max: Option<u64>,
    #[doc = "Conntrack established timeout."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_tcp_timeout_established: Option<u64>,
    #[doc = "Conntrack syn recv timeout."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nf_conntrack_tcp_timeout_syn_recv: Option<u64>,
    #[doc = "Enable SMURFS filter."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nosmurfs: Option<bool>,
    #[doc = "Enable synflood protection"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection_synflood: Option<bool>,
    #[doc = "Synflood protection rate burst by ip src."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protection_synflood_burst: Option<u64>,
    #[doc = "Synflood protection rate syn/sec by ip src."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protection_synflood_rate: Option<u64>,
    #[doc = "Log level for SMURFS filter."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smurf_log_level: Option<String>,
    #[doc = "Log level for illegal tcp flags filter."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tcp_flags_log_level: Option<String>,
    #[doc = "Filter illegal combinations of TCP flags."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tcpflags: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct OptionsClient<T> {
    client: T,
    path: String,
}

impl<T> OptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get host firewall options."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set Firewall options."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOptionsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> AsyncOptionsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get host firewall options."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set Firewall options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
