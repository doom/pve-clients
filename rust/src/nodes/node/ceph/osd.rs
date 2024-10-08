pub mod osdid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Set the device class of the OSD in crush."]
    #[serde(
        rename = "crush-device-class",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub crush_device_class: Option<String>,
    #[doc = "Block device name for block.db."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub db_dev: Option<String>,
    #[doc = "Size in GiB for block.db."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub db_dev_size: Option<f64>,
    #[doc = "Block device name."]
    pub dev: String,
    #[doc = "Enables encryption of the OSD."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub encrypted: Option<bool>,
    #[doc = "OSD services per physical device. Only useful for fast NVMe devices\" 		    .\" to utilize their performance better."]
    #[serde(
        rename = "osds-per-device",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub osds_per_device: Option<u64>,
    #[doc = "Block device name for block.wal."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wal_dev: Option<String>,
    #[doc = "Size in GiB for block.wal."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wal_dev_size: Option<f64>,
}

impl PostParameters {
    pub fn new(dev: String) -> Self {
        Self {
            dev,
            crush_device_class: Default::default(),
            db_dev: Default::default(),
            db_dev_size: Default::default(),
            encrypted: Default::default(),
            osds_per_device: Default::default(),
            wal_dev: Default::default(),
            wal_dev_size: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OsdClient<T> {
    client: T,
    path: String,
}

impl<T> OsdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "osd"),
        }
    }

    pub fn osdid(&self, osdid: &str) -> osdid::OsdidClient<T> {
        osdid::OsdidClient::<T>::new(self.client.clone(), &self.path, osdid)
    }
}
impl<T> OsdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get Ceph osd list/tree."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create OSD"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOsdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOsdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "osd"),
        }
    }

    pub fn osdid(&self, osdid: &str) -> osdid::AsyncOsdidClient<T> {
        osdid::AsyncOsdidClient::<T>::new(self.client.clone(), &self.path, osdid)
    }
}
impl<T> AsyncOsdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get Ceph osd list/tree."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create OSD"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
