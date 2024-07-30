pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "If given, checks the configurations on the given node for correctness, and adds relevant errors to the devices."]
    #[serde(
        rename = "check-node",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub check_node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "A description of the logical mapping."]
    pub description: String,
    #[doc = "A list of errors when 'check_node' is given."]
    pub error: serde_json::Value,
    #[doc = "The logical ID of the mapping."]
    pub id: String,
    #[doc = "The entries of the mapping."]
    pub map: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description of the logical USB device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "The ID of the logical USB mapping."]
    pub id: String,
    #[doc = "A list of maps for the cluster nodes."]
    pub map: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UsbClient<T> {
    client: T,
    path: String,
}

impl<T> UsbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "usb"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> UsbClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List USB Hardware Mappings"]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new hardware mapping."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUsbClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUsbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "usb"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncUsbClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List USB Hardware Mappings"]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new hardware mapping."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
