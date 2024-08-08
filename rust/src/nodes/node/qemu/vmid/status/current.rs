#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "QEMU Guest Agent is enabled in config."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub agent: Option<bool>,
    #[doc = "Enable a specific clipboard. If not set, depending on the display type the SPICE one will be added."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub clipboard: Option<String>,
    #[doc = "Maximum usable CPUs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpus: Option<f64>,
    #[doc = "HA manager service status."]
    pub ha: Ha,
    #[doc = "The current config lock, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Root disk size in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxdisk: Option<u64>,
    #[doc = "Maximum memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxmem: Option<u64>,
    #[doc = "VM name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "PID of running qemu process."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pid: Option<u64>,
    #[doc = "VM run state from the 'query-status' QMP monitor command."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub qmpstatus: Option<String>,
    #[doc = "The currently running machine type (if running)."]
    #[serde(
        rename = "running-machine",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub running_machine: Option<String>,
    #[doc = "The currently running QEMU version (if running)."]
    #[serde(
        rename = "running-qemu",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub running_qemu: Option<String>,
    #[doc = "QEMU VGA configuration supports spice."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub spice: Option<bool>,
    #[doc = "QEMU process status."]
    pub status: String,
    #[doc = "The current configured tags, if any"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Uptime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uptime: Option<u64>,
    #[doc = "The (unique) ID of the VM."]
    pub vmid: u64,
}

impl GetResponseItem {
    pub fn new(ha: Ha, status: String, vmid: u64) -> Self {
        Self {
            ha,
            status,
            vmid,
            agent: Default::default(),
            clipboard: Default::default(),
            cpus: Default::default(),
            lock: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            name: Default::default(),
            pid: Default::default(),
            qmpstatus: Default::default(),
            running_machine: Default::default(),
            running_qemu: Default::default(),
            spice: Default::default(),
            tags: Default::default(),
            uptime: Default::default(),
        }
    }
}

#[doc = "HA manager service status."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Ha {}

#[derive(Debug, Clone)]
pub struct CurrentClient<T> {
    client: T,
    path: String,
}

impl<T> CurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> CurrentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get virtual machine status."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCurrentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> AsyncCurrentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get virtual machine status."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
