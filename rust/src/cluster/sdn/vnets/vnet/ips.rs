#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    #[doc = "The IP address to delete"]
    pub ip: String,
    #[doc = "Unicast MAC address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mac: Option<String>,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The IP address to associate with the given MAC address"]
    pub ip: String,
    #[doc = "Unicast MAC address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mac: Option<String>,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "The IP address to associate with the given MAC address"]
    pub ip: String,
    #[doc = "Unicast MAC address."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mac: Option<String>,
    #[doc = "The (unique) ID of the VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
}

#[derive(Debug, Clone)]
pub struct IpsClient<T> {
    client: T,
    path: String,
}

impl<T> IpsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ips"),
        }
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete IP Mappings in a VNet"]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Create IP Mapping in a VNet"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update IP Mapping in a VNet"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIpsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIpsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ips"),
        }
    }
}
impl<T> AsyncIpsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete IP Mappings in a VNet"]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Create IP Mapping in a VNet"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update IP Mapping in a VNet"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
