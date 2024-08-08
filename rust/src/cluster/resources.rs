#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The cgroup mode the node operates under (when type == node)."]
    #[serde(
        rename = "cgroup-mode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub cgroup_mode: Option<u64>,
    #[doc = "Allowed storage content types (when type == storage)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[doc = "CPU utilization (when type in node,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpu: Option<f64>,
    #[doc = "Used disk space in bytes (when type in storage), used root image spave for VMs (type in qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub disk: Option<u64>,
    #[doc = "HA service status (for HA managed VMs)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hastate: Option<String>,
    #[doc = "Resource id."]
    pub id: String,
    #[doc = "Support level (when type == node)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub level: Option<String>,
    #[doc = "Number of available CPUs (when type in node,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxcpu: Option<f64>,
    #[doc = "Storage size in bytes (when type in storage), root image size for VMs (type in qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxdisk: Option<u64>,
    #[doc = "Number of available memory in bytes (when type in node,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxmem: Option<u64>,
    #[doc = "Used memory in bytes (when type in node,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mem: Option<u64>,
    #[doc = "Name of the resource."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "The cluster node name (when type in node,storage,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub node: Option<String>,
    #[doc = "More specific type, if available."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub plugintype: Option<String>,
    #[doc = "The pool name (when type in pool,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pool: Option<String>,
    #[doc = "Resource type dependent status."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    #[doc = "The storage identifier (when type == storage)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Resource type."]
    pub r#type: String,
    #[doc = "Node uptime in seconds (when type in node,qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uptime: Option<u64>,
    #[doc = "The numerical vmid (when type in qemu,lxc)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

impl GetResponseItem {
    pub fn new(id: String, r#type: String) -> Self {
        Self {
            id,
            r#type,
            cgroup_mode: Default::default(),
            content: Default::default(),
            cpu: Default::default(),
            disk: Default::default(),
            hastate: Default::default(),
            level: Default::default(),
            maxcpu: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            mem: Default::default(),
            name: Default::default(),
            node: Default::default(),
            plugintype: Default::default(),
            pool: Default::default(),
            status: Default::default(),
            storage: Default::default(),
            uptime: Default::default(),
            vmid: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourcesClient<T> {
    client: T,
    path: String,
}

impl<T> ResourcesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resources"),
        }
    }
}
impl<T> ResourcesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Resources index (cluster wide)."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncResourcesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncResourcesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resources"),
        }
    }
}
impl<T> AsyncResourcesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Resources index (cluster wide)."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
