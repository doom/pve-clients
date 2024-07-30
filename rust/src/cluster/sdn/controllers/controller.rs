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
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "autonomous system number"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub asn: Option<u64>,
    #[serde(
        rename = "bgp-multipath-as-path-relax",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub bgp_multipath_as_path_relax: Option<bool>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Enable ebgp. (remote-as external)"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ebgp: Option<bool>,
    #[serde(
        rename = "ebgp-multihop",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ebgp_multihop: Option<u64>,
    #[doc = "ISIS domain."]
    #[serde(
        rename = "isis-domain",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub isis_domain: Option<String>,
    #[doc = "ISIS interface."]
    #[serde(
        rename = "isis-ifaces",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub isis_ifaces: Option<String>,
    #[doc = "ISIS network entity title."]
    #[serde(rename = "isis-net", skip_serializing_if = "Option::is_none", default)]
    pub isis_net: Option<String>,
    #[doc = "source loopback interface."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub loopback: Option<String>,
    #[doc = "The cluster node name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub node: Option<String>,
    #[doc = "peers address list."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub peers: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ControllerClient<T> {
    client: T,
    path: String,
}

impl<T> ControllerClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, controller: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, controller),
        }
    }
}
impl<T> ControllerClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete sdn controller object configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read sdn controller configuration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Update sdn controller object configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncControllerClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncControllerClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, controller: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, controller),
        }
    }
}
impl<T> AsyncControllerClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete sdn controller object configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read sdn controller configuration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Update sdn controller object configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
