#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "The node for which the joinee gets the nodeinfo. "]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub config_digest: String,
    pub nodelist: Vec<GetResponseNodelistItem>,
    #[doc = "The cluster node name."]
    pub preferred_node: String,
    pub totem: Totem,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseNodelistItem {
    #[doc = "The cluster node name."]
    pub name: String,
    #[doc = "Node id for this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodeid: Option<u64>,
    pub pve_addr: String,
    #[doc = "Certificate SHA 256 fingerprint."]
    pub pve_fp: String,
    pub quorum_votes: u64,
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ring0_addr: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Totem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Certificate SHA 256 fingerprint."]
    pub fingerprint: String,
    #[doc = "Do not throw error if node already exists."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Hostname (or IP) of an existing cluster member."]
    pub hostname: String,
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_link_in_post_parameters",
        serialize_with = "serialize_repeated_link_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub links: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Node id for this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodeid: Option<u64>,
    #[doc = "Superuser (root) password of peer node."]
    pub password: String,
    #[doc = "Number of votes for this node"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub votes: Option<u64>,
}
pub fn deserialize_repeated_link_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("link", deserializer)
}

fn serialize_repeated_link_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "link", s)
}

#[derive(Debug, Clone)]
pub struct JoinClient<T> {
    client: T,
    path: String,
}

impl<T> JoinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "join"),
        }
    }
}
impl<T> JoinClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get information needed to join this cluster over the connected node."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links)."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncJoinClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncJoinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "join"),
        }
    }
}
impl<T> AsyncJoinClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get information needed to join this cluster over the connected node."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links)."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
