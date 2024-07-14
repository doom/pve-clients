#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "An API path prefix inserted between '<host>:<port>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy."]
    #[serde(
        rename = "api-path-prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub api_path_prefix: Option<String>,
    #[doc = "The InfluxDB bucket/db. Only necessary when using the http v2 api."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bucket: Option<String>,
    #[doc = "Flag to disable the plugin."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub influxdbproto: Option<String>,
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[serde(
        rename = "max-body-size",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_body_size: Option<u64>,
    #[doc = "MTU for metrics transmission over UDP"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtu: Option<u64>,
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization: Option<String>,
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,
    #[doc = "server network port"]
    pub port: u64,
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proto: Option<String>,
    #[doc = "server dns name or IP address"]
    pub server: String,
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
    #[doc = "The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    #[doc = "Plugin type."]
    pub r#type: String,
    #[doc = "Set to 0 to disable certificate verification for https endpoints."]
    #[serde(
        rename = "verify-certificate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verify_certificate: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "An API path prefix inserted between '<host>:<port>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy."]
    #[serde(
        rename = "api-path-prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub api_path_prefix: Option<String>,
    #[doc = "The InfluxDB bucket/db. Only necessary when using the http v2 api."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bucket: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Flag to disable the plugin."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub influxdbproto: Option<String>,
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[serde(
        rename = "max-body-size",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_body_size: Option<u64>,
    #[doc = "MTU for metrics transmission over UDP"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtu: Option<u64>,
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization: Option<String>,
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,
    #[doc = "server network port"]
    pub port: u64,
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proto: Option<String>,
    #[doc = "server dns name or IP address"]
    pub server: String,
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
    #[doc = "The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    #[doc = "Set to 0 to disable certificate verification for https endpoints."]
    #[serde(
        rename = "verify-certificate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verify_certificate: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}

impl<T> IdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove Metric server."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read metric server configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new external metric server config"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update metric server configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> AsyncIdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove Metric server."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read metric server configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new external metric server config"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update metric server configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
