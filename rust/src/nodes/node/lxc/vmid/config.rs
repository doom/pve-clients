#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Get current values (instead of pending values)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub current: Option<bool>,
    #[doc = "Fetch config values from given snapshot."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub snapshot: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "OS architecture type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
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
    #[doc = "SHA1 digest of configuration file. This can be used to prevent concurrent modifications."]
    pub digest: String,
    #[doc = "Allow containers access to advanced features."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub features: Option<String>,
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Set a host name for the container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hostname: Option<String>,
    #[doc = "Lock/unlock the container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Array of lxc low-level configurations ([[key1, value1], [key2, value2] ...])."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lxc: Option<Vec<Vec<String>>>,
    #[doc = "Amount of RAM for the container in MB."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<u64>,
    #[doc = "Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_mp_in_get_response_item",
        serialize_with = "serialize_repeated_mp_in_get_response_item",
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
        deserialize_with = "deserialize_repeated_net_in_get_response_item",
        serialize_with = "serialize_repeated_net_in_get_response_item",
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
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Use volume as container root."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rootfs: Option<String>,
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
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
        deserialize_with = "deserialize_repeated_unused_in_get_response_item",
        serialize_with = "serialize_repeated_unused_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
}
pub fn deserialize_repeated_mp_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("mp", deserializer)
}

fn serialize_repeated_mp_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "mp", s)
}

pub fn deserialize_repeated_net_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("net", deserializer)
}

fn serialize_repeated_net_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "net", s)
}

pub fn deserialize_repeated_unused_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("unused", deserializer)
}

fn serialize_repeated_unused_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "unused", s)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "OS architecture type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
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
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Allow containers access to advanced features."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub features: Option<String>,
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Set a host name for the container."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hostname: Option<String>,
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
        deserialize_with = "deserialize_repeated_mp_in_put_parameters",
        serialize_with = "serialize_repeated_mp_in_put_parameters",
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
        deserialize_with = "deserialize_repeated_net_in_put_parameters",
        serialize_with = "serialize_repeated_net_in_put_parameters",
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
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Revert a pending change."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revert: Option<String>,
    #[doc = "Use volume as container root."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rootfs: Option<String>,
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
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
        deserialize_with = "deserialize_repeated_unused_in_put_parameters",
        serialize_with = "serialize_repeated_unused_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
}
pub fn deserialize_repeated_mp_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("mp", deserializer)
}

fn serialize_repeated_mp_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "mp", s)
}

pub fn deserialize_repeated_net_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("net", deserializer)
}

fn serialize_repeated_net_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "net", s)
}

pub fn deserialize_repeated_unused_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("unused", deserializer)
}

fn serialize_repeated_unused_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "unused", s)
}

#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}

impl<T> ConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get container configuration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Set container options."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncConfigClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }
}
impl<T> AsyncConfigClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get container configuration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Set container options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
