#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[doc = "Enable ebtables rules cluster wide."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ebtables: Option<bool>,
    #[doc = "Enable or disable the firewall cluster wide."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable: Option<u64>,
    #[doc = "Log ratelimiting settings"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_ratelimit: Option<String>,
    #[doc = "Input policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_in: Option<String>,
    #[doc = "Output policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_out: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Enable ebtables rules cluster wide."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ebtables: Option<bool>,
    #[doc = "Enable or disable the firewall cluster wide."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable: Option<u64>,
    #[doc = "Log ratelimiting settings"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub log_ratelimit: Option<String>,
    #[doc = "Input policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_in: Option<String>,
    #[doc = "Output policy."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub policy_out: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OptionsClient<T> {
    client: T,
    path: String,
}

impl<T> OptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get Firewall options."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set Firewall options."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOptionsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> AsyncOptionsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get Firewall options."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set Firewall options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
