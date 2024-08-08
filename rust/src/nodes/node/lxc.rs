pub mod vmid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Maximum usable CPUs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpus: Option<f64>,
    #[doc = "The current config lock, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Root disk size in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxdisk: Option<u64>,
    #[doc = "Maximum memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxmem: Option<u64>,
    #[doc = "Maximum SWAP memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxswap: Option<u64>,
    #[doc = "Container name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "LXC Container status."]
    pub status: String,
    #[doc = "The current configured tags, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Uptime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uptime: Option<u64>,
    #[doc = "The (unique) ID of the VM."]
    pub vmid: u64,
}

impl GetResponseItem {
    pub fn new(status: String, vmid: u64) -> Self {
        Self {
            status,
            vmid,
            cpus: Default::default(),
            lock: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            maxswap: Default::default(),
            name: Default::default(),
            tags: Default::default(),
            uptime: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "OS architecture type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<f64>,
    #[doc = "Console mode. By default, the console command tries to open a connection to one of the available tty devices. By setting cmode to 'console' it tries to attach to /dev/console instead. If you set cmode to 'shell', it simply invokes a shell inside the container (no login)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cmode: Option<String>,
    #[doc = "Attach a console device (/dev/console) to the container."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub console: Option<bool>,
    #[doc = "The number of cores assigned to the container. A container can use all available cores by default."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cores: Option<u64>,
    #[doc = "Limit of CPU usage.  NOTE: If the computer has 2 CPUs, it has a total of '2' CPU time. Value '0' indicates no CPU limit."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpulimit: Option<f64>,
    #[doc = "CPU weight for a container, will be clamped to [1, 10000] in cgroup v2."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpuunits: Option<u64>,
    #[doc = "Try to be more verbose. For now this only enables debug log-level on start."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub debug: Option<bool>,
    #[doc = "Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Device to pass through to the container"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_dev_in_post_parameters",
        serialize_with = "serialize_repeated_dev_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub devs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Allow containers access to advanced features."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub features: Option<String>,
    #[doc = "Allow to overwrite existing container."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Set a host name for the container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hostname: Option<String>,
    #[doc = "Ignore errors when extracting the template."]
    #[serde(
        rename = "ignore-unpack-errors",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ignore_unpack_errors: Option<bool>,
    #[doc = "Lock/unlock the container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Amount of RAM for the container in MB."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<u64>,
    #[doc = "Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_mp_in_post_parameters",
        serialize_with = "serialize_repeated_mp_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub mps: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Sets DNS server IP address for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nameserver: Option<String>,
    #[doc = "Specifies network interfaces for the container."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_net_in_post_parameters",
        serialize_with = "serialize_repeated_net_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub nets: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Specifies whether a container will be started during system bootup."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub onboot: Option<bool>,
    #[doc = "The OS template or backup file."]
    pub ostemplate: String,
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Sets root password inside container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "Add the VM to the specified pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pool: Option<String>,
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Mark this as restore task."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub restore: Option<bool>,
    #[doc = "Use volume as container root."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rootfs: Option<String>,
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Setup public SSH keys (one key per line, OpenSSH format)."]
    #[serde(
        rename = "ssh-public-keys",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ssh_public_keys: Option<String>,
    #[doc = "Start the CT after its creation finished successfully."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub start: Option<bool>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
    #[doc = "Default Storage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Amount of SWAP for the container in MB."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub swap: Option<u64>,
    #[doc = "Tags of the Container. This is only meta information."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Enable/disable Template."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub template: Option<bool>,
    #[doc = "Time zone to use in the container. If option isn't set, then nothing will be done. Can be set to 'host' to match the host time zone, or an arbitrary time zone option from /usr/share/zoneinfo/zone.tab"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timezone: Option<String>,
    #[doc = "Specify the number of tty available to the container"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tty: Option<u64>,
    #[doc = "Assign a unique random ethernet address."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub unique: Option<bool>,
    #[doc = "Makes the container run as unprivileged user. (Should not be modified manually.)"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub unprivileged: Option<bool>,
    #[doc = "Reference to unused volumes. This is used internally, and should not be modified manually."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_unused_in_post_parameters",
        serialize_with = "serialize_repeated_unused_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
    #[doc = "The (unique) ID of the VM."]
    pub vmid: u64,
}

pub fn deserialize_repeated_dev_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("dev", deserializer)
}

fn serialize_repeated_dev_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "dev", s)
}

pub fn deserialize_repeated_mp_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("mp", deserializer)
}

fn serialize_repeated_mp_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "mp", s)
}

pub fn deserialize_repeated_net_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("net", deserializer)
}

fn serialize_repeated_net_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "net", s)
}

pub fn deserialize_repeated_unused_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("unused", deserializer)
}

fn serialize_repeated_unused_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "unused", s)
}

impl PostParameters {
    pub fn new(ostemplate: String, vmid: u64) -> Self {
        Self {
            ostemplate,
            vmid,
            arch: Default::default(),
            bwlimit: Default::default(),
            cmode: Default::default(),
            console: Default::default(),
            cores: Default::default(),
            cpulimit: Default::default(),
            cpuunits: Default::default(),
            debug: Default::default(),
            description: Default::default(),
            devs: Default::default(),
            features: Default::default(),
            force: Default::default(),
            hookscript: Default::default(),
            hostname: Default::default(),
            ignore_unpack_errors: Default::default(),
            lock: Default::default(),
            memory: Default::default(),
            mps: Default::default(),
            nameserver: Default::default(),
            nets: Default::default(),
            onboot: Default::default(),
            ostype: Default::default(),
            password: Default::default(),
            pool: Default::default(),
            protection: Default::default(),
            restore: Default::default(),
            rootfs: Default::default(),
            searchdomain: Default::default(),
            ssh_public_keys: Default::default(),
            start: Default::default(),
            startup: Default::default(),
            storage: Default::default(),
            swap: Default::default(),
            tags: Default::default(),
            template: Default::default(),
            timezone: Default::default(),
            tty: Default::default(),
            unique: Default::default(),
            unprivileged: Default::default(),
            unuseds: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LxcClient<T> {
    client: T,
    path: String,
}

impl<T> LxcClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lxc"),
        }
    }

    pub fn vmid(&self, vmid: &str) -> vmid::VmidClient<T> {
        vmid::VmidClient::<T>::new(self.client.clone(), &self.path, vmid)
    }
}
impl<T> LxcClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "LXC container index (per node)."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create or restore a container."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLxcClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLxcClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lxc"),
        }
    }

    pub fn vmid(&self, vmid: &str) -> vmid::AsyncVmidClient<T> {
        vmid::AsyncVmidClient::<T>::new(self.client.clone(), &self.path, vmid)
    }
}
impl<T> AsyncLxcClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "LXC container index (per node)."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create or restore a container."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
