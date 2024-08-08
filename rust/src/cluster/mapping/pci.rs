pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "If given, checks the configurations on the given node for correctness, and adds relevant diagnostics for the devices to the response."]
    #[serde(
        rename = "check-node",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub check_node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "A list of checks, only present if 'check_node' is set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checks: Option<Vec<GetResponseItemChecksItem>>,
    #[doc = "A description of the logical mapping."]
    pub description: String,
    #[doc = "The logical ID of the mapping."]
    pub id: String,
    #[doc = "The entries of the mapping."]
    pub map: Vec<String>,
}

impl GetResponseItem {
    pub fn new(description: String, id: String, map: Vec<String>) -> Self {
        Self {
            description,
            id,
            map,
            checks: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItemChecksItem {
    #[doc = "The message of the error"]
    pub message: String,
    #[doc = "The severity of the error"]
    pub severity: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description of the logical PCI device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "The ID of the logical PCI mapping."]
    pub id: String,
    #[doc = "A list of maps for the cluster nodes."]
    pub map: Vec<String>,
    #[doc = "Marks the device(s) as being capable of providing mediated devices."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mdev: Option<bool>,
}

impl PostParameters {
    pub fn new(id: String, map: Vec<String>) -> Self {
        Self {
            id,
            map,
            description: Default::default(),
            mdev: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PciClient<T> {
    client: T,
    path: String,
}

impl<T> PciClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pci"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> PciClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List PCI Hardware Mapping"]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new hardware mapping."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPciClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPciClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pci"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncPciClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List PCI Hardware Mapping"]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new hardware mapping."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
