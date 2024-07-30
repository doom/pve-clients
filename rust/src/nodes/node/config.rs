#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Return only a specific property from the node configuration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub property: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[doc = "Node specific ACME settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub acme: Option<String>,
    #[doc = "ACME domain and validation plugin"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_acmedomain_in_get_response_item",
        serialize_with = "serialize_repeated_acmedomain_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub acmedomains: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[serde(
        rename = "startall-onboot-delay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub startall_onboot_delay: Option<u64>,
    #[doc = "Node specific wake on LAN settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wakeonlan: Option<String>,
}
pub fn deserialize_repeated_acmedomain_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("acmedomain", deserializer)
}

fn serialize_repeated_acmedomain_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "acmedomain", s)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Node specific ACME settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub acme: Option<String>,
    #[doc = "ACME domain and validation plugin"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_acmedomain_in_put_parameters",
        serialize_with = "serialize_repeated_acmedomain_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub acmedomains: std::collections::HashMap<u32, Option<String>>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[serde(
        rename = "startall-onboot-delay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub startall_onboot_delay: Option<u64>,
    #[doc = "Node specific wake on LAN settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wakeonlan: Option<String>,
}
pub fn deserialize_repeated_acmedomain_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("acmedomain", deserializer)
}

fn serialize_repeated_acmedomain_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "acmedomain", s)
}

#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}

impl<T> ConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get node configuration options."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Set node configuration options."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncConfigClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> AsyncConfigClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get node configuration options."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Set node configuration options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
