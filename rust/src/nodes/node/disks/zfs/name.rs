#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only)."]
    #[serde(
        rename = "cleanup-config",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub cleanup_config: Option<bool>,
    #[doc = "Also wipe disks so they can be repurposed afterwards."]
    #[serde(
        rename = "cleanup-disks",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub cleanup_disks: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Information about the recommended action to fix the state."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub action: Option<String>,
    #[doc = "The pool configuration information, including the vdevs for each section (e.g. spares, cache), may be nested."]
    pub children: Vec<GetResponseChildrenItem>,
    #[doc = "Information about the errors on the zpool."]
    pub errors: String,
    #[doc = "The name of the zpool."]
    pub name: String,
    #[doc = "Information about the last/current scrub."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scan: Option<String>,
    #[doc = "The state of the zpool."]
    pub state: String,
    #[doc = "Information about the state of the zpool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseChildrenItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cksum: Option<f64>,
    #[doc = "An optional message about the vdev."]
    pub msg: String,
    #[doc = "The name of the vdev or section."]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub read: Option<f64>,
    #[doc = "The state of the vdev."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub write: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy a ZFS pool."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Get details about a zpool."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy a ZFS pool."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Get details about a zpool."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
