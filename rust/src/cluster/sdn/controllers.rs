pub mod controller;

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
    #[doc = "Only list sdn controllers of specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub controller: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
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
    #[doc = "The SDN controller object identifier."]
    pub controller: String,
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
    #[doc = "Plugin type."]
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct ControllersClient<T> {
    client: T,
    path: String,
}

impl<T> ControllersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "controllers"),
        }
    }

    pub fn controller(&self, controller: &str) -> controller::ControllerClient<T> {
        controller::ControllerClient::<T>::new(self.client.clone(), &self.path, controller)
    }
}
impl<T> ControllersClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN controllers index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn controller object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncControllersClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncControllersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "controllers"),
        }
    }

    pub fn controller(&self, controller: &str) -> controller::AsyncControllerClient<T> {
        controller::AsyncControllerClient::<T>::new(self.client.clone(), &self.path, controller)
    }
}
impl<T> AsyncControllersClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN controllers index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn controller object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
