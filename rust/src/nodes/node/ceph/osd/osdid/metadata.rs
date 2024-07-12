#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Array containing data about devices"]
    pub devices: Vec<GetResponseDevicesItem>,
    #[doc = "General information about the OSD"]
    pub osd: Osd,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseDevicesItem {
    #[doc = "Device node"]
    pub dev_node: String,
    #[doc = "Kind of OSD device"]
    pub device: String,
    #[doc = "Physical disks used"]
    pub devices: String,
    #[doc = "Size in bytes"]
    pub size: u64,
    #[doc = "Discard support of the physical device"]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub support_discard: bool,
    #[doc = "Type of device. For example, hdd or ssd"]
    pub r#type: String,
}

#[doc = "General information about the OSD"]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Osd {
    #[doc = "Address and port used to talk to other OSDs."]
    pub back_addr: String,
    #[doc = "Address and port used to talk to clients and monitors."]
    pub front_addr: String,
    #[doc = "Heartbeat address and port for other OSDs."]
    pub hb_back_addr: String,
    #[doc = "Heartbeat address and port for clients and monitors."]
    pub hb_front_addr: String,
    #[doc = "Name of the host containing the OSD."]
    pub hostname: String,
    #[doc = "ID of the OSD."]
    pub id: u64,
    #[doc = "Memory usage of the OSD service."]
    pub mem_usage: u64,
    #[doc = "Path to the OSD's data directory."]
    pub osd_data: String,
    #[doc = "The type of object store used."]
    pub osd_objectstore: String,
    #[doc = "OSD process ID."]
    pub pid: u64,
    #[doc = "Ceph version of the OSD service."]
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct MetadataClient<T> {
    client: T,
    path: String,
}

impl<T> MetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metadata"),
        }
    }
}
impl<T> MetadataClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get OSD details"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metadata"),
        }
    }
}
impl<T> AsyncMetadataClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get OSD details"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
