#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "For type 'service'. Service state as seen by the CRM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crm_state: Option<String>,
    #[doc = "Status entry ID (quorum, master, lrm:<node>, service:<sid>)."]
    pub id: String,
    #[doc = "For type 'service'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_relocate: Option<u64>,
    #[doc = "For type 'service'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_restart: Option<u64>,
    #[doc = "Node associated to status entry."]
    pub node: String,
    #[doc = "For type 'quorum'. Whether the cluster is quorate or not."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub quorate: Option<bool>,
    #[doc = "For type 'service'. Requested service state."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub request_state: Option<String>,
    #[doc = "For type 'service'. Service ID."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sid: Option<String>,
    #[doc = "For type 'service'. Verbose service state."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    #[doc = "Status of the entry (value depends on type)."]
    pub status: String,
    #[doc = "For type 'lrm','master'. Timestamp of the status information."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timestamp: Option<u64>,
    #[doc = "Type of status entry."]
    pub r#type: serde_json::Value,
}

impl GetResponseItem {
    pub fn new(id: String, node: String, status: String, r#type: serde_json::Value) -> Self {
        Self {
            id,
            node,
            status,
            r#type,
            crm_state: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            quorate: Default::default(),
            request_state: Default::default(),
            sid: Default::default(),
            state: Default::default(),
            timestamp: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentClient<T> {
    client: T,
    path: String,
}

impl<T> CurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> CurrentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get HA manger status."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCurrentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> AsyncCurrentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get HA manger status."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
