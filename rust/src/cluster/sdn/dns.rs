pub mod dns;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list sdn dns of specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub dns: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The SDN dns object identifier."]
    pub dns: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reversemaskv6: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reversev6mask: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ttl: Option<u64>,
    #[doc = "Plugin type."]
    pub r#type: String,
    pub url: String,
}

impl PostParameters {
    pub fn new(dns: String, key: String, r#type: String, url: String) -> Self {
        Self {
            dns,
            key,
            r#type,
            url,
            reversemaskv6: Default::default(),
            reversev6mask: Default::default(),
            ttl: Default::default(),
        }
    }
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

    pub fn dns(&self, dns: &str) -> dns::DnsClient<T> {
        dns::DnsClient::<T>::new(self.client.clone(), &self.path, dns)
    }
}
impl<T> DnsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN dns index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn dns object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
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

    pub fn dns(&self, dns: &str) -> dns::AsyncDnsClient<T> {
        dns::AsyncDnsClient::<T>::new(self.client.clone(), &self.path, dns)
    }
}
impl<T> AsyncDnsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN dns index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn dns object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
