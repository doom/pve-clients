pub mod poolid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    pub poolid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub poolid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub members: Option<Vec<GetResponseItemMembersItem>>,
    pub poolid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItemMembersItem {
    pub id: String,
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub poolid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Allow adding a guest even if already in another pool. The guest will be removed from its current pool and added to this one."]
    #[serde(
        rename = "allow-move",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub allow_move: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Remove the passed VMIDs and/or storage IDs instead of adding them."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub delete: Option<bool>,
    pub poolid: String,
    #[doc = "List of storage IDs to add or remove from this pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "List of guest VMIDs to add or remove from this pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
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
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "pools".to_string(),
        }
    }

    pub fn poolid(&self, poolid: &str) -> poolid::PoolidClient<T> {
        poolid::PoolidClient::<T>::new(self.client.clone(), &self.path, poolid)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete pool."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "List pools or get pool configuration."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create new pool."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update pool."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
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
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "pools".to_string(),
        }
    }

    pub fn poolid(&self, poolid: &str) -> poolid::AsyncPoolidClient<T> {
        poolid::AsyncPoolidClient::<T>::new(self.client.clone(), &self.path, poolid)
    }
}
impl<T> AsyncPoolsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete pool."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "List pools or get pool configuration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create new pool."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update pool."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
