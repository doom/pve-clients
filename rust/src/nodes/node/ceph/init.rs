#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Declare a separate cluster network, OSDs will routeheartbeat, object replication and recovery traffic over it"]
    #[serde(
        rename = "cluster-network",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub cluster_network: Option<String>,
    #[doc = "Disable cephx authentication.  WARNING: cephx is a security feature protecting against man-in-the-middle attacks. Only consider disabling cephx if your network is private!"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable_cephx: Option<bool>,
    #[doc = "Minimum number of available replicas per object to allow I/O"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_size: Option<u64>,
    #[doc = "Use specific network for all ceph related traffic"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub network: Option<String>,
    #[doc = "Placement group bits, used to specify the default number of placement groups.  NOTE: 'osd pool default pg num' does not work for default pools."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_bits: Option<u64>,
    #[doc = "Targeted number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct InitClient<T> {
    client: T,
    path: String,
}

impl<T> InitClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "init"),
        }
    }
}
impl<T> InitClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Create initial ceph default configuration and setup symlinks."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInitClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInitClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "init"),
        }
    }
}
impl<T> AsyncInitClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Create initial ceph default configuration and setup symlinks."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
