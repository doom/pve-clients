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

#[doc = "The VM configuration."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Enable/disable ACPI."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub acpi: Option<bool>,
    #[doc = "List of host cores used to execute guest processes, for example: 0,5,8-11"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub affinity: Option<String>,
    #[doc = "Enable/disable communication with the QEMU Guest Agent and its properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent: Option<String>,
    #[doc = "Virtual processor architecture. Defaults to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
    #[doc = "Arbitrary arguments passed to kvm."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub args: Option<String>,
    #[doc = "Configure a audio device, useful in combination with QXL/Spice."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub audio0: Option<String>,
    #[doc = "Automatic restart after crash (currently ignored)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub autostart: Option<bool>,
    #[doc = "Amount of target RAM for the VM in MiB. Using zero disables the ballon driver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub balloon: Option<u64>,
    #[doc = "Select BIOS implementation."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bios: Option<String>,
    #[doc = "Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub boot: Option<String>,
    #[doc = "Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bootdisk: Option<String>,
    #[doc = "This is an alias for option -ide2"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cdrom: Option<String>,
    #[doc = "cloud-init: Specify custom files to replace the automatically generated ones at start."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cicustom: Option<String>,
    #[doc = "cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cipassword: Option<String>,
    #[doc = "Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub citype: Option<String>,
    #[doc = "cloud-init: do an automatic package upgrade after the first boot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ciupgrade: Option<bool>,
    #[doc = "cloud-init: User name to change ssh keys and password for instead of the image's configured default user."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ciuser: Option<String>,
    #[doc = "The number of cores per socket."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cores: Option<u64>,
    #[doc = "Emulated CPU type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpu: Option<String>,
    #[doc = "Limit of CPU usage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpulimit: Option<f64>,
    #[doc = "CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpuunits: Option<u64>,
    #[doc = "Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "SHA1 digest of configuration file. This can be used to prevent concurrent modifications."]
    pub digest: String,
    #[doc = "Configure a disk for storing EFI vars."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub efidisk0: Option<String>,
    #[doc = "Freeze CPU at startup (use 'c' monitor command to start execution)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub freeze: Option<bool>,
    #[doc = "Script that will be executed during various steps in the vms lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Map host PCI devices into guest."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_hostpci_in_get_response_item",
        serialize_with = "serialize_repeated_hostpci_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub hostpcis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hotplug: Option<String>,
    #[doc = "Enable/disable hugepages memory."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hugepages: Option<String>,
    #[doc = "Use volume as IDE hard disk or CD-ROM (n is 0 to 3)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ide_in_get_response_item",
        serialize_with = "serialize_repeated_ide_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ides: std::collections::HashMap<u32, Option<String>>,
    #[doc = "cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4. "]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ipconfig_in_get_response_item",
        serialize_with = "serialize_repeated_ipconfig_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ipconfigs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Inter-VM shared memory. Useful for direct communication between VMs, or to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ivshmem: Option<String>,
    #[doc = "Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub keephugepages: Option<bool>,
    #[doc = "Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keyboard: Option<String>,
    #[doc = "Enable/disable KVM hardware virtualization."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub kvm: Option<bool>,
    #[doc = "Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub localtime: Option<bool>,
    #[doc = "Lock/unlock the VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Specify the QEMU machine."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub machine: Option<String>,
    #[doc = "Memory properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<String>,
    #[doc = "Set maximum tolerated downtime (in seconds) for migrations. Should the migration not be able to converge in the very end, because too much newly dirtied RAM needs to be transferred, the limit will be increased automatically step-by-step until migration can converge."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_downtime: Option<f64>,
    #[doc = "Set maximum speed (in MB/s) for migrations. Value 0 is no limit."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_speed: Option<u64>,
    #[doc = "Set a name for the VM. Only used on the configuration web interface."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nameserver: Option<String>,
    #[doc = "Specify network devices."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_net_in_get_response_item",
        serialize_with = "serialize_repeated_net_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub nets: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Enable/disable NUMA."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub numa: Option<bool>,
    #[doc = "NUMA topology."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_numa_in_get_response_item",
        serialize_with = "serialize_repeated_numa_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub numas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Specifies whether a VM will be started during system bootup."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub onboot: Option<bool>,
    #[doc = "Specify guest operating system."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Map host parallel devices (n is 0 to 2)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_parallel_in_get_response_item",
        serialize_with = "serialize_repeated_parallel_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub parallels: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Sets the protection flag of the VM. This will disable the remove VM and remove disk operations."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Allow reboot. If set to '0' the VM exit on reboot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub reboot: Option<bool>,
    #[doc = "Configure a VirtIO-based Random Number Generator."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rng0: Option<String>,
    #[doc = "Use volume as SATA hard disk or CD-ROM (n is 0 to 5)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_sata_in_get_response_item",
        serialize_with = "serialize_repeated_sata_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub satas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Use volume as SCSI hard disk or CD-ROM (n is 0 to 30)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_scsi_in_get_response_item",
        serialize_with = "serialize_repeated_scsi_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub scsis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "SCSI controller model"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scsihw: Option<String>,
    #[doc = "cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Create a serial device inside the VM (n is 0 to 3)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_serial_in_get_response_item",
        serialize_with = "serialize_repeated_serial_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub serials: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub shares: Option<u64>,
    #[doc = "Specify SMBIOS type 1 fields."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smbios1: Option<String>,
    #[doc = "The number of CPUs. Please use option -sockets instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smp: Option<u64>,
    #[doc = "The number of CPU sockets."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sockets: Option<u64>,
    #[doc = "Configure additional enhancements for SPICE."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spice_enhancements: Option<String>,
    #[doc = "cloud-init: Setup public SSH keys (one key per line, OpenSSH format)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sshkeys: Option<String>,
    #[doc = "Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startdate: Option<String>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
    #[doc = "Enable/disable the USB tablet device."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tablet: Option<bool>,
    #[doc = "Tags of the VM. This is only meta information."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Enable/disable time drift fix."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tdf: Option<bool>,
    #[doc = "Enable/disable Template."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub template: Option<bool>,
    #[doc = "Configure a Disk for storing TPM state. The format is fixed to 'raw'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tpmstate0: Option<String>,
    #[doc = "Reference to unused volumes. This is used internally, and should not be modified manually."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_unused_in_get_response_item",
        serialize_with = "serialize_repeated_unused_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_usb_in_get_response_item",
        serialize_with = "serialize_repeated_usb_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub usbs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Number of hotplugged vcpus."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vcpus: Option<u64>,
    #[doc = "Configure the VGA hardware."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vga: Option<String>,
    #[doc = "Use volume as VIRTIO hard disk (n is 0 to 15)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_virtio_in_get_response_item",
        serialize_with = "serialize_repeated_virtio_in_get_response_item",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub virtios: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmgenid: Option<String>,
    #[doc = "Default storage for VM state volumes/files."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmstatestorage: Option<String>,
    #[doc = "Create a virtual hardware watchdog device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub watchdog: Option<String>,
}

pub fn deserialize_repeated_hostpci_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("hostpci", deserializer)
}

fn serialize_repeated_hostpci_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "hostpci", s)
}

pub fn deserialize_repeated_ide_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ide", deserializer)
}

fn serialize_repeated_ide_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ide", s)
}

pub fn deserialize_repeated_ipconfig_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ipconfig", deserializer)
}

fn serialize_repeated_ipconfig_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ipconfig", s)
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

pub fn deserialize_repeated_numa_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("numa", deserializer)
}

fn serialize_repeated_numa_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "numa", s)
}

pub fn deserialize_repeated_parallel_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("parallel", deserializer)
}

fn serialize_repeated_parallel_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "parallel", s)
}

pub fn deserialize_repeated_sata_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("sata", deserializer)
}

fn serialize_repeated_sata_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "sata", s)
}

pub fn deserialize_repeated_scsi_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("scsi", deserializer)
}

fn serialize_repeated_scsi_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "scsi", s)
}

pub fn deserialize_repeated_serial_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("serial", deserializer)
}

fn serialize_repeated_serial_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "serial", s)
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

pub fn deserialize_repeated_usb_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("usb", deserializer)
}

fn serialize_repeated_usb_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "usb", s)
}

pub fn deserialize_repeated_virtio_in_get_response_item<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("virtio", deserializer)
}

fn serialize_repeated_virtio_in_get_response_item<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "virtio", s)
}

impl GetResponseItem {
    pub fn new(digest: String) -> Self {
        Self {
            digest,
            acpi: Default::default(),
            affinity: Default::default(),
            agent: Default::default(),
            arch: Default::default(),
            args: Default::default(),
            audio0: Default::default(),
            autostart: Default::default(),
            balloon: Default::default(),
            bios: Default::default(),
            boot: Default::default(),
            bootdisk: Default::default(),
            cdrom: Default::default(),
            cicustom: Default::default(),
            cipassword: Default::default(),
            citype: Default::default(),
            ciupgrade: Default::default(),
            ciuser: Default::default(),
            cores: Default::default(),
            cpu: Default::default(),
            cpulimit: Default::default(),
            cpuunits: Default::default(),
            description: Default::default(),
            efidisk0: Default::default(),
            freeze: Default::default(),
            hookscript: Default::default(),
            hostpcis: Default::default(),
            hotplug: Default::default(),
            hugepages: Default::default(),
            ides: Default::default(),
            ipconfigs: Default::default(),
            ivshmem: Default::default(),
            keephugepages: Default::default(),
            keyboard: Default::default(),
            kvm: Default::default(),
            localtime: Default::default(),
            lock: Default::default(),
            machine: Default::default(),
            memory: Default::default(),
            migrate_downtime: Default::default(),
            migrate_speed: Default::default(),
            name: Default::default(),
            nameserver: Default::default(),
            nets: Default::default(),
            numa: Default::default(),
            numas: Default::default(),
            onboot: Default::default(),
            ostype: Default::default(),
            parallels: Default::default(),
            protection: Default::default(),
            reboot: Default::default(),
            rng0: Default::default(),
            satas: Default::default(),
            scsis: Default::default(),
            scsihw: Default::default(),
            searchdomain: Default::default(),
            serials: Default::default(),
            shares: Default::default(),
            smbios1: Default::default(),
            smp: Default::default(),
            sockets: Default::default(),
            spice_enhancements: Default::default(),
            sshkeys: Default::default(),
            startdate: Default::default(),
            startup: Default::default(),
            tablet: Default::default(),
            tags: Default::default(),
            tdf: Default::default(),
            template: Default::default(),
            tpmstate0: Default::default(),
            unuseds: Default::default(),
            usbs: Default::default(),
            vcpus: Default::default(),
            vga: Default::default(),
            virtios: Default::default(),
            vmgenid: Default::default(),
            vmstatestorage: Default::default(),
            watchdog: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Enable/disable ACPI."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub acpi: Option<bool>,
    #[doc = "List of host cores used to execute guest processes, for example: 0,5,8-11"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub affinity: Option<String>,
    #[doc = "Enable/disable communication with the QEMU Guest Agent and its properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent: Option<String>,
    #[doc = "Virtual processor architecture. Defaults to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
    #[doc = "Arbitrary arguments passed to kvm."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub args: Option<String>,
    #[doc = "Configure a audio device, useful in combination with QXL/Spice."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub audio0: Option<String>,
    #[doc = "Automatic restart after crash (currently ignored)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub autostart: Option<bool>,
    #[doc = "Time to wait for the task to finish. We return 'null' if the task finish within that time."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub background_delay: Option<u64>,
    #[doc = "Amount of target RAM for the VM in MiB. Using zero disables the ballon driver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub balloon: Option<u64>,
    #[doc = "Select BIOS implementation."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bios: Option<String>,
    #[doc = "Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub boot: Option<String>,
    #[doc = "Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bootdisk: Option<String>,
    #[doc = "This is an alias for option -ide2"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cdrom: Option<String>,
    #[doc = "cloud-init: Specify custom files to replace the automatically generated ones at start."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cicustom: Option<String>,
    #[doc = "cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cipassword: Option<String>,
    #[doc = "Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub citype: Option<String>,
    #[doc = "cloud-init: do an automatic package upgrade after the first boot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ciupgrade: Option<bool>,
    #[doc = "cloud-init: User name to change ssh keys and password for instead of the image's configured default user."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ciuser: Option<String>,
    #[doc = "The number of cores per socket."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cores: Option<u64>,
    #[doc = "Emulated CPU type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpu: Option<String>,
    #[doc = "Limit of CPU usage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpulimit: Option<f64>,
    #[doc = "CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpuunits: Option<u64>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Configure a disk for storing EFI vars. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and that the default EFI vars are copied to the volume instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub efidisk0: Option<String>,
    #[doc = "Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Freeze CPU at startup (use 'c' monitor command to start execution)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub freeze: Option<bool>,
    #[doc = "Script that will be executed during various steps in the vms lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Map host PCI devices into guest."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_hostpci_in_post_parameters",
        serialize_with = "serialize_repeated_hostpci_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub hostpcis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hotplug: Option<String>,
    #[doc = "Enable/disable hugepages memory."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hugepages: Option<String>,
    #[doc = "Use volume as IDE hard disk or CD-ROM (n is 0 to 3). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ide_in_post_parameters",
        serialize_with = "serialize_repeated_ide_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ides: std::collections::HashMap<u32, Option<String>>,
    #[doc = "cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4. "]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ipconfig_in_post_parameters",
        serialize_with = "serialize_repeated_ipconfig_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ipconfigs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Inter-VM shared memory. Useful for direct communication between VMs, or to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ivshmem: Option<String>,
    #[doc = "Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub keephugepages: Option<bool>,
    #[doc = "Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keyboard: Option<String>,
    #[doc = "Enable/disable KVM hardware virtualization."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub kvm: Option<bool>,
    #[doc = "Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub localtime: Option<bool>,
    #[doc = "Lock/unlock the VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Specify the QEMU machine."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub machine: Option<String>,
    #[doc = "Memory properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<String>,
    #[doc = "Set maximum tolerated downtime (in seconds) for migrations. Should the migration not be able to converge in the very end, because too much newly dirtied RAM needs to be transferred, the limit will be increased automatically step-by-step until migration can converge."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_downtime: Option<f64>,
    #[doc = "Set maximum speed (in MB/s) for migrations. Value 0 is no limit."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_speed: Option<u64>,
    #[doc = "Set a name for the VM. Only used on the configuration web interface."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nameserver: Option<String>,
    #[doc = "Specify network devices."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_net_in_post_parameters",
        serialize_with = "serialize_repeated_net_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub nets: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Enable/disable NUMA."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub numa: Option<bool>,
    #[doc = "NUMA topology."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_numa_in_post_parameters",
        serialize_with = "serialize_repeated_numa_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub numas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Specifies whether a VM will be started during system bootup."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub onboot: Option<bool>,
    #[doc = "Specify guest operating system."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Map host parallel devices (n is 0 to 2)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_parallel_in_post_parameters",
        serialize_with = "serialize_repeated_parallel_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub parallels: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Sets the protection flag of the VM. This will disable the remove VM and remove disk operations."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Allow reboot. If set to '0' the VM exit on reboot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub reboot: Option<bool>,
    #[doc = "Revert a pending change."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revert: Option<String>,
    #[doc = "Configure a VirtIO-based Random Number Generator."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rng0: Option<String>,
    #[doc = "Use volume as SATA hard disk or CD-ROM (n is 0 to 5). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_sata_in_post_parameters",
        serialize_with = "serialize_repeated_sata_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub satas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Use volume as SCSI hard disk or CD-ROM (n is 0 to 30). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_scsi_in_post_parameters",
        serialize_with = "serialize_repeated_scsi_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub scsis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "SCSI controller model"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scsihw: Option<String>,
    #[doc = "cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Create a serial device inside the VM (n is 0 to 3)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_serial_in_post_parameters",
        serialize_with = "serialize_repeated_serial_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub serials: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub shares: Option<u64>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
    #[doc = "Specify SMBIOS type 1 fields."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smbios1: Option<String>,
    #[doc = "The number of CPUs. Please use option -sockets instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smp: Option<u64>,
    #[doc = "The number of CPU sockets."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sockets: Option<u64>,
    #[doc = "Configure additional enhancements for SPICE."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spice_enhancements: Option<String>,
    #[doc = "cloud-init: Setup public SSH keys (one key per line, OpenSSH format)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sshkeys: Option<String>,
    #[doc = "Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startdate: Option<String>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
    #[doc = "Enable/disable the USB tablet device."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tablet: Option<bool>,
    #[doc = "Tags of the VM. This is only meta information."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Enable/disable time drift fix."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tdf: Option<bool>,
    #[doc = "Enable/disable Template."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub template: Option<bool>,
    #[doc = "Configure a Disk for storing TPM state. The format is fixed to 'raw'. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and 4 MiB will be used instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tpmstate0: Option<String>,
    #[doc = "Reference to unused volumes. This is used internally, and should not be modified manually."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_unused_in_post_parameters",
        serialize_with = "serialize_repeated_unused_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_usb_in_post_parameters",
        serialize_with = "serialize_repeated_usb_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub usbs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Number of hotplugged vcpus."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vcpus: Option<u64>,
    #[doc = "Configure the VGA hardware."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vga: Option<String>,
    #[doc = "Use volume as VIRTIO hard disk (n is 0 to 15). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_virtio_in_post_parameters",
        serialize_with = "serialize_repeated_virtio_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub virtios: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmgenid: Option<String>,
    #[doc = "Default storage for VM state volumes/files."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmstatestorage: Option<String>,
    #[doc = "Create a virtual hardware watchdog device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub watchdog: Option<String>,
}

pub fn deserialize_repeated_hostpci_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("hostpci", deserializer)
}

fn serialize_repeated_hostpci_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "hostpci", s)
}

pub fn deserialize_repeated_ide_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ide", deserializer)
}

fn serialize_repeated_ide_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ide", s)
}

pub fn deserialize_repeated_ipconfig_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ipconfig", deserializer)
}

fn serialize_repeated_ipconfig_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ipconfig", s)
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

pub fn deserialize_repeated_numa_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("numa", deserializer)
}

fn serialize_repeated_numa_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "numa", s)
}

pub fn deserialize_repeated_parallel_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("parallel", deserializer)
}

fn serialize_repeated_parallel_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "parallel", s)
}

pub fn deserialize_repeated_sata_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("sata", deserializer)
}

fn serialize_repeated_sata_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "sata", s)
}

pub fn deserialize_repeated_scsi_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("scsi", deserializer)
}

fn serialize_repeated_scsi_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "scsi", s)
}

pub fn deserialize_repeated_serial_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("serial", deserializer)
}

fn serialize_repeated_serial_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "serial", s)
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

pub fn deserialize_repeated_usb_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("usb", deserializer)
}

fn serialize_repeated_usb_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "usb", s)
}

pub fn deserialize_repeated_virtio_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("virtio", deserializer)
}

fn serialize_repeated_virtio_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "virtio", s)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Enable/disable ACPI."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub acpi: Option<bool>,
    #[doc = "List of host cores used to execute guest processes, for example: 0,5,8-11"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub affinity: Option<String>,
    #[doc = "Enable/disable communication with the QEMU Guest Agent and its properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent: Option<String>,
    #[doc = "Virtual processor architecture. Defaults to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub arch: Option<String>,
    #[doc = "Arbitrary arguments passed to kvm."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub args: Option<String>,
    #[doc = "Configure a audio device, useful in combination with QXL/Spice."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub audio0: Option<String>,
    #[doc = "Automatic restart after crash (currently ignored)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub autostart: Option<bool>,
    #[doc = "Amount of target RAM for the VM in MiB. Using zero disables the ballon driver."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub balloon: Option<u64>,
    #[doc = "Select BIOS implementation."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bios: Option<String>,
    #[doc = "Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub boot: Option<String>,
    #[doc = "Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bootdisk: Option<String>,
    #[doc = "This is an alias for option -ide2"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cdrom: Option<String>,
    #[doc = "cloud-init: Specify custom files to replace the automatically generated ones at start."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cicustom: Option<String>,
    #[doc = "cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cipassword: Option<String>,
    #[doc = "Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub citype: Option<String>,
    #[doc = "cloud-init: do an automatic package upgrade after the first boot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub ciupgrade: Option<bool>,
    #[doc = "cloud-init: User name to change ssh keys and password for instead of the image's configured default user."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ciuser: Option<String>,
    #[doc = "The number of cores per socket."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cores: Option<u64>,
    #[doc = "Emulated CPU type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpu: Option<String>,
    #[doc = "Limit of CPU usage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpulimit: Option<f64>,
    #[doc = "CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpuunits: Option<u64>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Configure a disk for storing EFI vars. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and that the default EFI vars are copied to the volume instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub efidisk0: Option<String>,
    #[doc = "Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "Freeze CPU at startup (use 'c' monitor command to start execution)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub freeze: Option<bool>,
    #[doc = "Script that will be executed during various steps in the vms lifetime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hookscript: Option<String>,
    #[doc = "Map host PCI devices into guest."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_hostpci_in_put_parameters",
        serialize_with = "serialize_repeated_hostpci_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub hostpcis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hotplug: Option<String>,
    #[doc = "Enable/disable hugepages memory."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hugepages: Option<String>,
    #[doc = "Use volume as IDE hard disk or CD-ROM (n is 0 to 3). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ide_in_put_parameters",
        serialize_with = "serialize_repeated_ide_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ides: std::collections::HashMap<u32, Option<String>>,
    #[doc = "cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4. "]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_ipconfig_in_put_parameters",
        serialize_with = "serialize_repeated_ipconfig_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ipconfigs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Inter-VM shared memory. Useful for direct communication between VMs, or to the host."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ivshmem: Option<String>,
    #[doc = "Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub keephugepages: Option<bool>,
    #[doc = "Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keyboard: Option<String>,
    #[doc = "Enable/disable KVM hardware virtualization."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub kvm: Option<bool>,
    #[doc = "Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub localtime: Option<bool>,
    #[doc = "Lock/unlock the VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Specify the QEMU machine."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub machine: Option<String>,
    #[doc = "Memory properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub memory: Option<String>,
    #[doc = "Set maximum tolerated downtime (in seconds) for migrations. Should the migration not be able to converge in the very end, because too much newly dirtied RAM needs to be transferred, the limit will be increased automatically step-by-step until migration can converge."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_downtime: Option<f64>,
    #[doc = "Set maximum speed (in MB/s) for migrations. Value 0 is no limit."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migrate_speed: Option<u64>,
    #[doc = "Set a name for the VM. Only used on the configuration web interface."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nameserver: Option<String>,
    #[doc = "Specify network devices."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_net_in_put_parameters",
        serialize_with = "serialize_repeated_net_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub nets: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Enable/disable NUMA."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub numa: Option<bool>,
    #[doc = "NUMA topology."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_numa_in_put_parameters",
        serialize_with = "serialize_repeated_numa_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub numas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Specifies whether a VM will be started during system bootup."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub onboot: Option<bool>,
    #[doc = "Specify guest operating system."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ostype: Option<String>,
    #[doc = "Map host parallel devices (n is 0 to 2)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_parallel_in_put_parameters",
        serialize_with = "serialize_repeated_parallel_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub parallels: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Sets the protection flag of the VM. This will disable the remove VM and remove disk operations."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protection: Option<bool>,
    #[doc = "Allow reboot. If set to '0' the VM exit on reboot."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub reboot: Option<bool>,
    #[doc = "Revert a pending change."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revert: Option<String>,
    #[doc = "Configure a VirtIO-based Random Number Generator."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rng0: Option<String>,
    #[doc = "Use volume as SATA hard disk or CD-ROM (n is 0 to 5). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_sata_in_put_parameters",
        serialize_with = "serialize_repeated_sata_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub satas: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Use volume as SCSI hard disk or CD-ROM (n is 0 to 30). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_scsi_in_put_parameters",
        serialize_with = "serialize_repeated_scsi_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub scsis: std::collections::HashMap<u32, Option<String>>,
    #[doc = "SCSI controller model"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scsihw: Option<String>,
    #[doc = "cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub searchdomain: Option<String>,
    #[doc = "Create a serial device inside the VM (n is 0 to 3)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_serial_in_put_parameters",
        serialize_with = "serialize_repeated_serial_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub serials: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub shares: Option<u64>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
    #[doc = "Specify SMBIOS type 1 fields."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smbios1: Option<String>,
    #[doc = "The number of CPUs. Please use option -sockets instead."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smp: Option<u64>,
    #[doc = "The number of CPU sockets."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sockets: Option<u64>,
    #[doc = "Configure additional enhancements for SPICE."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spice_enhancements: Option<String>,
    #[doc = "cloud-init: Setup public SSH keys (one key per line, OpenSSH format)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sshkeys: Option<String>,
    #[doc = "Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startdate: Option<String>,
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startup: Option<String>,
    #[doc = "Enable/disable the USB tablet device."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tablet: Option<bool>,
    #[doc = "Tags of the VM. This is only meta information."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Enable/disable time drift fix."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tdf: Option<bool>,
    #[doc = "Enable/disable Template."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub template: Option<bool>,
    #[doc = "Configure a Disk for storing TPM state. The format is fixed to 'raw'. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and 4 MiB will be used instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tpmstate0: Option<String>,
    #[doc = "Reference to unused volumes. This is used internally, and should not be modified manually."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_unused_in_put_parameters",
        serialize_with = "serialize_repeated_unused_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub unuseds: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14)."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_usb_in_put_parameters",
        serialize_with = "serialize_repeated_usb_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub usbs: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Number of hotplugged vcpus."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vcpus: Option<u64>,
    #[doc = "Configure the VGA hardware."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vga: Option<String>,
    #[doc = "Use volume as VIRTIO hard disk (n is 0 to 15). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_virtio_in_put_parameters",
        serialize_with = "serialize_repeated_virtio_in_put_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub virtios: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmgenid: Option<String>,
    #[doc = "Default storage for VM state volumes/files."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmstatestorage: Option<String>,
    #[doc = "Create a virtual hardware watchdog device."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub watchdog: Option<String>,
}

pub fn deserialize_repeated_hostpci_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("hostpci", deserializer)
}

fn serialize_repeated_hostpci_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "hostpci", s)
}

pub fn deserialize_repeated_ide_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ide", deserializer)
}

fn serialize_repeated_ide_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ide", s)
}

pub fn deserialize_repeated_ipconfig_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("ipconfig", deserializer)
}

fn serialize_repeated_ipconfig_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "ipconfig", s)
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

pub fn deserialize_repeated_numa_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("numa", deserializer)
}

fn serialize_repeated_numa_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "numa", s)
}

pub fn deserialize_repeated_parallel_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("parallel", deserializer)
}

fn serialize_repeated_parallel_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "parallel", s)
}

pub fn deserialize_repeated_sata_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("sata", deserializer)
}

fn serialize_repeated_sata_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "sata", s)
}

pub fn deserialize_repeated_scsi_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("scsi", deserializer)
}

fn serialize_repeated_scsi_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "scsi", s)
}

pub fn deserialize_repeated_serial_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("serial", deserializer)
}

fn serialize_repeated_serial_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "serial", s)
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

pub fn deserialize_repeated_usb_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("usb", deserializer)
}

fn serialize_repeated_usb_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "usb", s)
}

pub fn deserialize_repeated_virtio_in_put_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("virtio", deserializer)
}

fn serialize_repeated_virtio_in_put_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "virtio", s)
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
    #[doc = "Get the virtual machine configuration with pending configuration changes applied. Set the 'current' parameter to get the current configuration instead."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Set virtual machine options (asynchrounous API)."]
    pub fn post(&self, parameters: PostParameters) -> Result<Option<String>, T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Set virtual machine options (synchrounous API) - You should consider using the POST method instead for any actions involving hotplug or storage allocation."]
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
    #[doc = "Get the virtual machine configuration with pending configuration changes applied. Set the 'current' parameter to get the current configuration instead."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Set virtual machine options (asynchrounous API)."]
    pub async fn post(&self, parameters: PostParameters) -> Result<Option<String>, T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Set virtual machine options (synchrounous API) - You should consider using the POST method instead for any actions involving hotplug or storage allocation."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
