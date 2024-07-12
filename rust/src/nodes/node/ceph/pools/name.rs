#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    #[doc = "If true, destroys pool even if in use"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Remove the erasure code profile. Defaults to true, if applicable."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub remove_ecprofile: Option<bool>,
    #[doc = "Remove all pveceph-managed storages configured for this pool"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub remove_storages: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "If enabled, will display additional data(eg. statistics)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verbose: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The application of the pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoscale_status: Option<AutoscaleStatus>,
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crush_rule: Option<String>,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub fast_read: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub hashpspool: bool,
    pub id: u64,
    #[doc = "Minimum number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_size: Option<u64>,
    #[doc = "The name of the pool. It must be unique."]
    pub name: String,
    #[serde(
        rename = "nodeep-scrub",
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub nodeep_scrub: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub nodelete: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub nopgchange: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub noscrub: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub nosizechange: bool,
    #[doc = "The automatic PG scaling mode of the pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_autoscale_mode: Option<String>,
    #[doc = "Number of placement groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num: Option<u64>,
    #[doc = "Minimal number of placement groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_min: Option<u64>,
    pub pgp_num: u64,
    #[doc = "Number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statistics: Option<Statistics>,
    #[doc = "The estimated target size of the pool for the PG autoscaler."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size: Option<String>,
    #[doc = "The estimated target ratio of the pool for the PG autoscaler."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size_ratio: Option<f64>,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub use_gmt_hitset: bool,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub write_fadvise_dontneed: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AutoscaleStatus {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Statistics {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "The application of the pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application: Option<String>,
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crush_rule: Option<String>,
    #[doc = "Minimum number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_size: Option<u64>,
    #[doc = "The automatic PG scaling mode of the pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_autoscale_mode: Option<String>,
    #[doc = "Number of placement groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num: Option<u64>,
    #[doc = "Minimal number of placement groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_min: Option<u64>,
    #[doc = "Number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<u64>,
    #[doc = "The estimated target size of the pool for the PG autoscaler."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size: Option<String>,
    #[doc = "The estimated target ratio of the pool for the PG autoscaler."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size_ratio: Option<f64>,
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
    #[doc = "Destroy pool. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "List pool settings. Deprecated, please use `/nodes/{node}/ceph/pool/{pool}/status`."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Change POOL settings. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
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
    #[doc = "Destroy pool. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "List pool settings. Deprecated, please use `/nodes/{node}/ceph/pool/{pool}/status`."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Change POOL settings. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
