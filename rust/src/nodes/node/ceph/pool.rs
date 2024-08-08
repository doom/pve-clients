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

impl GetResponseItem {
    pub fn new(
        bytes_used: u64,
        crush_rule: u64,
        crush_rule_name: String,
        min_size: u64,
        percent_used: f64,
        pg_num: u64,
        pool: u64,
        pool_name: String,
        size: u64,
        r#type: String,
    ) -> Self {
        Self {
            bytes_used,
            crush_rule,
            crush_rule_name,
            min_size,
            percent_used,
            pg_num,
            pool,
            pool_name,
            size,
            r#type,
            application_metadata: Default::default(),
            autoscale_status: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num_final: Default::default(),
            pg_num_min: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
        }
    }
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

impl PostParameters {
    pub fn new(name: String) -> Self {
        Self {
            name,
            add_storages: Default::default(),
            application: Default::default(),
            crush_rule: Default::default(),
            erasure_coding: Default::default(),
            min_size: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num: Default::default(),
            pg_num_min: Default::default(),
            size: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolClient<T> {
    client: T,
    path: String,
}

impl<T> PoolClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pool"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> PoolClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List all pools and their settings (which are settable by the POST/PUT endpoints)."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create Ceph pool"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPoolClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPoolClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pool"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncPoolClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List all pools and their settings (which are settable by the POST/PUT endpoints)."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create Ceph pool"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
