#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Feature to check."]
    pub feature: String,
    #[doc = "The name of the snapshot."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub snapname: Option<String>,
}

impl GetParameters {
    pub fn new(feature: String) -> Self {
        Self {
            feature,
            snapname: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(
        rename = "hasFeature",
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub has_feature: bool,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FeatureClient<T> {
    client: T,
    path: String,
}

impl<T> FeatureClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "feature"),
        }
    }
}
impl<T> FeatureClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Check if feature for virtual machine is available."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFeatureClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFeatureClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "feature"),
        }
    }
}
impl<T> AsyncFeatureClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Check if feature for virtual machine is available."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
