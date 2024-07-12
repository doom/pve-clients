#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Enable DHCP."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub dhcp: Option<bool>,
    #[doc = "Enable/disable firewall rules."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Enable default IP filters. This is equivalent to adding an empty ipfilter-net<id> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ipfilter: Option<bool>,
    #[doc = "Log level for incoming traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_in: Option<String>,
    #[doc = "Log level for outgoing traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_out: Option<String>,
    #[doc = "Enable/disable MAC address filter."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub macfilter: Option<bool>,
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ndp: Option<bool>,
    #[doc = "Input policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_in: Option<String>,
    #[doc = "Output policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_out: Option<String>,
    #[doc = "Allow sending Router Advertisement."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub radv: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Enable DHCP."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub dhcp: Option<bool>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Enable/disable firewall rules."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Enable default IP filters. This is equivalent to adding an empty ipfilter-net<id> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ipfilter: Option<bool>,
    #[doc = "Log level for incoming traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_in: Option<String>,
    #[doc = "Log level for outgoing traffic."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_level_out: Option<String>,
    #[doc = "Enable/disable MAC address filter."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub macfilter: Option<bool>,
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ndp: Option<bool>,
    #[doc = "Input policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_in: Option<String>,
    #[doc = "Output policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_out: Option<String>,
    #[doc = "Allow sending Router Advertisement."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub radv: Option<bool>,
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
    #[doc = "Get VM firewall options."]
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
    #[doc = "Get VM firewall options."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set Firewall options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
