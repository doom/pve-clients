pub mod pciid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "A list of blacklisted PCI classes, which will not be returned. Following are filtered by default: Memory Controller (05), Bridge (06) and Processor (0b)."]
    #[serde(
        rename = "pci-class-blacklist",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pci_class_blacklist: Option<String>,
    #[doc = "If disabled, does only print the PCI IDs. Otherwise, additional information like vendor and device will be returned."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verbose: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The PCI Class of the device."]
    pub class: String,
    #[doc = "The Device ID."]
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub device_name: Option<String>,
    #[doc = "The PCI ID."]
    pub id: String,
    #[doc = "The IOMMU group in which the device is in. If no IOMMU group is detected, it is set to -1."]
    pub iommugroup: u64,
    #[doc = "If set, marks that the device is capable of creating mediated devices."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mdev: Option<bool>,
    #[doc = "The Subsystem Device ID."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_device_name: Option<String>,
    #[doc = "The Subsystem Vendor ID."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_vendor_name: Option<String>,
    #[doc = "The Vendor ID."]
    pub vendor: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vendor_name: Option<String>,
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

    pub fn pciid(&self, pciid: &str) -> pciid::PciidClient<T> {
        pciid::PciidClient::<T>::new(self.client.clone(), &self.path, pciid)
    }
}
impl<T> PciClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List local PCI devices."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
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

    pub fn pciid(&self, pciid: &str) -> pciid::AsyncPciidClient<T> {
        pciid::AsyncPciidClient::<T>::new(self.client.clone(), &self.path, pciid)
    }
}
impl<T> AsyncPciClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List local PCI devices."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
