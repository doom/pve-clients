#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[doc = "First name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns1: Option<String>,
    #[doc = "Second name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns2: Option<String>,
    #[doc = "Third name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns3: Option<String>,
    #[doc = "Search domain for host-name lookup."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub search: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "First name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns1: Option<String>,
    #[doc = "Second name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns2: Option<String>,
    #[doc = "Third name server IP address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dns3: Option<String>,
    #[doc = "Search domain for host-name lookup."]
    pub search: String,
}

#[derive(Debug, Clone)]
pub struct DnsClient<T> {
    client: T,
    path: String,
}

impl<T> DnsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "dns"),
        }
    }
}
impl<T> DnsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read DNS settings."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Write DNS settings."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDnsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDnsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "dns"),
        }
    }
}
impl<T> AsyncDnsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read DNS settings."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Write DNS settings."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
