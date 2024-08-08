#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
    #[doc = "[node] IP of the resolved nodename."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ip: Option<String>,
    #[doc = "[node] Proxmox VE Subscription level, indicates if eligible for enterprise support as well as access to the stable Proxmox VE Enterprise Repository."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub level: Option<String>,
    #[doc = "[node] Indicates if this is the responding node."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub local: Option<bool>,
    pub name: String,
    #[doc = "[node] ID of the node from the corosync configuration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodeid: Option<u64>,
    #[doc = "[cluster] Nodes count, including offline nodes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<u64>,
    #[doc = "[node] Indicates if the node is online or offline."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub online: Option<bool>,
    #[doc = "[cluster] Indicates if there is a majority of nodes online to make decisions"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub quorate: Option<bool>,
    #[doc = "Indicates the type, either cluster or node. The type defines the object properties e.g. quorate available for type cluster."]
    pub r#type: String,
    #[doc = "[cluster] Current version of the corosync configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<u64>,
}

impl GetResponseItem {
    pub fn new(id: String, name: String, r#type: String) -> Self {
        Self {
            id,
            name,
            r#type,
            ip: Default::default(),
            level: Default::default(),
            local: Default::default(),
            nodeid: Default::default(),
            nodes: Default::default(),
            online: Default::default(),
            quorate: Default::default(),
            version: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}

impl<T> StatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get cluster status information."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get cluster status information."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
