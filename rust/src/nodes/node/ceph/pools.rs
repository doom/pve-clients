pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application_metadata: Option<ApplicationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoscale_status: Option<AutoscaleStatus>,
    pub bytes_used: u64,
    pub crush_rule: u64,
    pub crush_rule_name: String,
    pub min_size: u64,
    pub percent_used: f64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_autoscale_mode: Option<String>,
    pub pg_num: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_final: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pg_num_min: Option<u64>,
    pub pool: u64,
    pub pool_name: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_size_ratio: Option<f64>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ApplicationMetadata {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct AutoscaleStatus {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Configure VM and CT storage using the new pool."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub add_storages: Option<bool>,
    #[doc = "The application of the pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub application: Option<String>,
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crush_rule: Option<String>,
    #[doc = "Create an erasure coded pool for RBD with an accompaning replicated pool for metadata storage. With EC, the common ceph options 'size', 'min_size' and 'crush_rule' parameters will be applied to the metadata pool."]
    #[serde(
        rename = "erasure-coding",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub erasure_coding: Option<String>,
    #[doc = "Minimum number of replicas per object"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub min_size: Option<u64>,
    #[doc = "The name of the pool. It must be unique."]
    pub name: String,
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
pub struct PoolsClient<T> {
    client: T,
    path: String,
}

impl<T> PoolsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pools"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List all pools. Deprecated, please use `/nodes/{node}/ceph/pool`."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create Ceph pool. Deprecated, please use `/nodes/{node}/ceph/pool`."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPoolsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPoolsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pools"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncPoolsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List all pools. Deprecated, please use `/nodes/{node}/ceph/pool`."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create Ceph pool. Deprecated, please use `/nodes/{node}/ceph/pool`."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
