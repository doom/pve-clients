#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "The JOIN_API_VERSION of the new node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub apiversion: Option<u64>,
    #[doc = "Do not throw error if node already exists."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_link_in_post_parameters",
        serialize_with = "serialize_repeated_link_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub links: std::collections::HashMap<u32, Option<String>>,
    #[doc = "IP Address of node to add. Used as fallback if no links are given."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub new_node_ip: Option<String>,
    #[doc = "Node id for this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodeid: Option<u64>,
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub corosync_authkey: String,
    pub corosync_conf: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NodeClient<T> {
    client: T,
    path: String,
}

impl<T> NodeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Removes a node from the cluster configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Adds a node to the cluster configuration. This call is for internal use."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNodeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNodeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }
}
impl<T> AsyncNodeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Removes a node from the cluster configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Adds a node to the cluster configuration. This call is for internal use."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
