#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub members: Vec<GetResponseMembersItem>,
}

impl GetResponseItem {
    pub fn new(members: Vec<GetResponseMembersItem>) -> Self {
        Self {
            members,
            comment: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseMembersItem {
    pub id: String,
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

impl GetResponseMembersItem {
    pub fn new(id: String, node: String, r#type: String) -> Self {
        Self {
            id,
            node,
            r#type,
            storage: Default::default(),
            vmid: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
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
    #[doc = "List of storage IDs to add or remove from this pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "List of guest VMIDs to add or remove from this pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PoolidClient<T> {
    client: T,
    path: String,
}

impl<T> PoolidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, poolid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, poolid),
        }
    }
}
impl<T> PoolidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete pool (deprecated, no support for nested pools, use 'DELETE /pools/?poolid={poolid}')."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get pool configuration (deprecated, no support for nested pools, use 'GET /pools/?poolid={poolid}')."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Update pool data (deprecated, no support for nested pools - use 'PUT /pools/?poolid={poolid}' instead)."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPoolidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPoolidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, poolid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, poolid),
        }
    }
}
impl<T> AsyncPoolidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete pool (deprecated, no support for nested pools, use 'DELETE /pools/?poolid={poolid}')."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get pool configuration (deprecated, no support for nested pools, use 'GET /pools/?poolid={poolid}')."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Update pool data (deprecated, no support for nested pools - use 'PUT /pools/?poolid={poolid}' instead)."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
