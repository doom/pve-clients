#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reversemaskv6: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ttl: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
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
    pub fn new(client: T, parent_path: &str, dns: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, dns),
        }
    }
}
impl<T> DnsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete sdn dns object configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read sdn dns configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update sdn dns object configuration."]
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
    pub fn new(client: T, parent_path: &str, dns: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, dns),
        }
    }
}
impl<T> AsyncDnsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete sdn dns object configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read sdn dns configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update sdn dns object configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
