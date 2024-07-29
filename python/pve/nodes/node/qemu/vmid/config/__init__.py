from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Enable/disable ACPI.
    acpi: Optional[bool] = Field(default=None)
    # List of host cores used to execute guest processes, for example: 0,5,8-11
    affinity: Optional[str] = Field(default=None)
    # Enable/disable communication with the QEMU Guest Agent and its properties.
    agent: Optional[str] = Field(default=None)
    # Virtual processor architecture. Defaults to the host.
    arch: Optional[str] = Field(default=None)
    # Arbitrary arguments passed to kvm.
    args: Optional[str] = Field(default=None)
    # Configure a audio device, useful in combination with QXL/Spice.
    audio0: Optional[str] = Field(default=None)
    # Automatic restart after crash (currently ignored).
    autostart: Optional[bool] = Field(default=None)
    # Amount of target RAM for the VM in MiB. Using zero disables the ballon driver.
    balloon: Optional[int] = Field(default=None)
    # Select BIOS implementation.
    bios: Optional[str] = Field(default=None)
    # Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated.
    boot: Optional[str] = Field(default=None)
    # Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead.
    bootdisk: Optional[str] = Field(default=None)
    # This is an alias for option -ide2
    cdrom: Optional[str] = Field(default=None)
    # cloud-init: Specify custom files to replace the automatically generated ones at start.
    cicustom: Optional[str] = Field(default=None)
    # cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords.
    cipassword: Optional[str] = Field(default=None)
    # Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows.
    citype: Optional[str] = Field(default=None)
    # cloud-init: User name to change ssh keys and password for instead of the image's configured default user.
    ciuser: Optional[str] = Field(default=None)
    # The number of cores per socket.
    cores: Optional[int] = Field(default=None)
    # Emulated CPU type.
    cpu: Optional[str] = Field(default=None)
    # Limit of CPU usage.
    cpulimit: Optional[float] = Field(default=None)
    # CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2.
    cpuunits: Optional[int] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Configure a disk for storing EFI vars. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and that the default EFI vars are copied to the volume instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    efidisk0: Optional[str] = Field(default=None)
    # Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal.
    force: Optional[bool] = Field(default=None)
    # Freeze CPU at startup (use 'c' monitor command to start execution).
    freeze: Optional[bool] = Field(default=None)
    # Script that will be executed during various steps in the vms lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Map host PCI devices into guest.
    hostpcis: dict[int, Optional[str]] = Field(default=None)
    # Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7.
    hotplug: Optional[str] = Field(default=None)
    # Enable/disable hugepages memory.
    hugepages: Optional[str] = Field(default=None)
    # Use volume as IDE hard disk or CD-ROM (n is 0 to 3). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    ides: dict[int, Optional[str]] = Field(default=None)
    # cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4.
    ipconfigs: dict[int, Optional[str]] = Field(default=None)
    # Inter-VM shared memory. Useful for direct communication between VMs, or to the host.
    ivshmem: Optional[str] = Field(default=None)
    # Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts.
    keephugepages: Optional[bool] = Field(default=None)
    # Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS.
    keyboard: Optional[str] = Field(default=None)
    # Enable/disable KVM hardware virtualization.
    kvm: Optional[bool] = Field(default=None)
    # Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS.
    localtime: Optional[bool] = Field(default=None)
    # Lock/unlock the VM.
    lock: Optional[str] = Field(default=None)
    # Specifies the QEMU machine type.
    machine: Optional[str] = Field(default=None)
    # Amount of RAM for the VM in MiB. This is the maximum available memory when you use the balloon device.
    memory: Optional[int] = Field(default=None)
    # Set maximum tolerated downtime (in seconds) for migrations.
    migrate_downtime: Optional[float] = Field(default=None)
    # Set maximum speed (in MB/s) for migrations. Value 0 is no limit.
    migrate_speed: Optional[int] = Field(default=None)
    # Set a name for the VM. Only used on the configuration web interface.
    name: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    nameserver: Optional[str] = Field(default=None)
    # Specify network devices.
    nets: dict[int, Optional[str]] = Field(default=None)
    # Enable/disable NUMA.
    numa: Optional[bool] = Field(default=None)
    # NUMA topology.
    numas: dict[int, Optional[str]] = Field(default=None)
    # Specifies whether a VM will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # Specify guest operating system.
    ostype: Optional[str] = Field(default=None)
    # Map host parallel devices (n is 0 to 2).
    parallels: dict[int, Optional[str]] = Field(default=None)
    # Sets the protection flag of the VM. This will disable the remove VM and remove disk operations.
    protection: Optional[bool] = Field(default=None)
    # Allow reboot. If set to '0' the VM exit on reboot.
    reboot: Optional[bool] = Field(default=None)
    # Revert a pending change.
    revert: Optional[str] = Field(default=None)
    # Configure a VirtIO-based Random Number Generator.
    rng0: Optional[str] = Field(default=None)
    # Use volume as SATA hard disk or CD-ROM (n is 0 to 5). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    satas: dict[int, Optional[str]] = Field(default=None)
    # Use volume as SCSI hard disk or CD-ROM (n is 0 to 30). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    scsis: dict[int, Optional[str]] = Field(default=None)
    # SCSI controller model
    scsihw: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    searchdomain: Optional[str] = Field(default=None)
    # Create a serial device inside the VM (n is 0 to 3)
    serials: dict[int, Optional[str]] = Field(default=None)
    # Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd.
    shares: Optional[int] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # Specify SMBIOS type 1 fields.
    smbios1: Optional[str] = Field(default=None)
    # The number of CPUs. Please use option -sockets instead.
    smp: Optional[int] = Field(default=None)
    # The number of CPU sockets.
    sockets: Optional[int] = Field(default=None)
    # Configure additional enhancements for SPICE.
    spice_enhancements: Optional[str] = Field(default=None)
    # cloud-init: Setup public SSH keys (one key per line, OpenSSH format).
    sshkeys: Optional[str] = Field(default=None)
    # Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'.
    startdate: Optional[str] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
    # Enable/disable the USB tablet device.
    tablet: Optional[bool] = Field(default=None)
    # Tags of the VM. This is only meta information.
    tags: Optional[str] = Field(default=None)
    # Enable/disable time drift fix.
    tdf: Optional[bool] = Field(default=None)
    # Enable/disable Template.
    template: Optional[bool] = Field(default=None)
    # Configure a Disk for storing TPM state. The format is fixed to 'raw'. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and 4 MiB will be used instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    tpmstate0: Optional[str] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(default=None)
    # Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14).
    usbs: dict[int, Optional[str]] = Field(default=None)
    # Number of hotplugged vcpus.
    vcpus: Optional[int] = Field(default=None)
    # Configure the VGA hardware.
    vga: Optional[str] = Field(default=None)
    # Use volume as VIRTIO hard disk (n is 0 to 15). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    virtios: dict[int, Optional[str]] = Field(default=None)
    # Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly.
    vmgenid: Optional[str] = Field(default=None)
    # Default storage for VM state volumes/files.
    vmstatestorage: Optional[str] = Field(default=None)
    # Create a virtual hardware watchdog device.
    watchdog: Optional[str] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="hostpcis", prefix="hostpci")
        data = serialize_repeated_with_prefix(data, group="ides", prefix="ide")
        data = serialize_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = serialize_repeated_with_prefix(data, group="nets", prefix="net")
        data = serialize_repeated_with_prefix(data, group="numas", prefix="numa")
        data = serialize_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = serialize_repeated_with_prefix(data, group="satas", prefix="sata")
        data = serialize_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = serialize_repeated_with_prefix(data, group="serials", prefix="serial")
        data = serialize_repeated_with_prefix(data, group="unuseds", prefix="unused")
        data = serialize_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = serialize_repeated_with_prefix(data, group="virtios", prefix="virtio")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(
            data, group="hostpcis", prefix="hostpci"
        )
        data = data = extract_repeated_with_prefix(data, group="ides", prefix="ide")
        data = data = extract_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = data = extract_repeated_with_prefix(data, group="nets", prefix="net")
        data = data = extract_repeated_with_prefix(data, group="numas", prefix="numa")
        data = data = extract_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = data = extract_repeated_with_prefix(data, group="satas", prefix="sata")
        data = data = extract_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = data = extract_repeated_with_prefix(
            data, group="serials", prefix="serial"
        )
        data = data = extract_repeated_with_prefix(
            data, group="unuseds", prefix="unused"
        )
        data = data = extract_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = data = extract_repeated_with_prefix(
            data, group="virtios", prefix="virtio"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Enable/disable ACPI.
    acpi: Optional[bool] = Field(default=None)
    # List of host cores used to execute guest processes, for example: 0,5,8-11
    affinity: Optional[str] = Field(default=None)
    # Enable/disable communication with the QEMU Guest Agent and its properties.
    agent: Optional[str] = Field(default=None)
    # Virtual processor architecture. Defaults to the host.
    arch: Optional[str] = Field(default=None)
    # Arbitrary arguments passed to kvm.
    args: Optional[str] = Field(default=None)
    # Configure a audio device, useful in combination with QXL/Spice.
    audio0: Optional[str] = Field(default=None)
    # Automatic restart after crash (currently ignored).
    autostart: Optional[bool] = Field(default=None)
    # Time to wait for the task to finish. We return 'null' if the task finish within that time.
    background_delay: Optional[int] = Field(default=None)
    # Amount of target RAM for the VM in MiB. Using zero disables the ballon driver.
    balloon: Optional[int] = Field(default=None)
    # Select BIOS implementation.
    bios: Optional[str] = Field(default=None)
    # Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated.
    boot: Optional[str] = Field(default=None)
    # Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead.
    bootdisk: Optional[str] = Field(default=None)
    # This is an alias for option -ide2
    cdrom: Optional[str] = Field(default=None)
    # cloud-init: Specify custom files to replace the automatically generated ones at start.
    cicustom: Optional[str] = Field(default=None)
    # cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords.
    cipassword: Optional[str] = Field(default=None)
    # Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows.
    citype: Optional[str] = Field(default=None)
    # cloud-init: User name to change ssh keys and password for instead of the image's configured default user.
    ciuser: Optional[str] = Field(default=None)
    # The number of cores per socket.
    cores: Optional[int] = Field(default=None)
    # Emulated CPU type.
    cpu: Optional[str] = Field(default=None)
    # Limit of CPU usage.
    cpulimit: Optional[float] = Field(default=None)
    # CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2.
    cpuunits: Optional[int] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Configure a disk for storing EFI vars. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and that the default EFI vars are copied to the volume instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    efidisk0: Optional[str] = Field(default=None)
    # Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal.
    force: Optional[bool] = Field(default=None)
    # Freeze CPU at startup (use 'c' monitor command to start execution).
    freeze: Optional[bool] = Field(default=None)
    # Script that will be executed during various steps in the vms lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Map host PCI devices into guest.
    hostpcis: dict[int, Optional[str]] = Field(default=None)
    # Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7.
    hotplug: Optional[str] = Field(default=None)
    # Enable/disable hugepages memory.
    hugepages: Optional[str] = Field(default=None)
    # Use volume as IDE hard disk or CD-ROM (n is 0 to 3). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    ides: dict[int, Optional[str]] = Field(default=None)
    # cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4.
    ipconfigs: dict[int, Optional[str]] = Field(default=None)
    # Inter-VM shared memory. Useful for direct communication between VMs, or to the host.
    ivshmem: Optional[str] = Field(default=None)
    # Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts.
    keephugepages: Optional[bool] = Field(default=None)
    # Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS.
    keyboard: Optional[str] = Field(default=None)
    # Enable/disable KVM hardware virtualization.
    kvm: Optional[bool] = Field(default=None)
    # Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS.
    localtime: Optional[bool] = Field(default=None)
    # Lock/unlock the VM.
    lock: Optional[str] = Field(default=None)
    # Specifies the QEMU machine type.
    machine: Optional[str] = Field(default=None)
    # Amount of RAM for the VM in MiB. This is the maximum available memory when you use the balloon device.
    memory: Optional[int] = Field(default=None)
    # Set maximum tolerated downtime (in seconds) for migrations.
    migrate_downtime: Optional[float] = Field(default=None)
    # Set maximum speed (in MB/s) for migrations. Value 0 is no limit.
    migrate_speed: Optional[int] = Field(default=None)
    # Set a name for the VM. Only used on the configuration web interface.
    name: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    nameserver: Optional[str] = Field(default=None)
    # Specify network devices.
    nets: dict[int, Optional[str]] = Field(default=None)
    # Enable/disable NUMA.
    numa: Optional[bool] = Field(default=None)
    # NUMA topology.
    numas: dict[int, Optional[str]] = Field(default=None)
    # Specifies whether a VM will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # Specify guest operating system.
    ostype: Optional[str] = Field(default=None)
    # Map host parallel devices (n is 0 to 2).
    parallels: dict[int, Optional[str]] = Field(default=None)
    # Sets the protection flag of the VM. This will disable the remove VM and remove disk operations.
    protection: Optional[bool] = Field(default=None)
    # Allow reboot. If set to '0' the VM exit on reboot.
    reboot: Optional[bool] = Field(default=None)
    # Revert a pending change.
    revert: Optional[str] = Field(default=None)
    # Configure a VirtIO-based Random Number Generator.
    rng0: Optional[str] = Field(default=None)
    # Use volume as SATA hard disk or CD-ROM (n is 0 to 5). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    satas: dict[int, Optional[str]] = Field(default=None)
    # Use volume as SCSI hard disk or CD-ROM (n is 0 to 30). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    scsis: dict[int, Optional[str]] = Field(default=None)
    # SCSI controller model
    scsihw: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    searchdomain: Optional[str] = Field(default=None)
    # Create a serial device inside the VM (n is 0 to 3)
    serials: dict[int, Optional[str]] = Field(default=None)
    # Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd.
    shares: Optional[int] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # Specify SMBIOS type 1 fields.
    smbios1: Optional[str] = Field(default=None)
    # The number of CPUs. Please use option -sockets instead.
    smp: Optional[int] = Field(default=None)
    # The number of CPU sockets.
    sockets: Optional[int] = Field(default=None)
    # Configure additional enhancements for SPICE.
    spice_enhancements: Optional[str] = Field(default=None)
    # cloud-init: Setup public SSH keys (one key per line, OpenSSH format).
    sshkeys: Optional[str] = Field(default=None)
    # Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'.
    startdate: Optional[str] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
    # Enable/disable the USB tablet device.
    tablet: Optional[bool] = Field(default=None)
    # Tags of the VM. This is only meta information.
    tags: Optional[str] = Field(default=None)
    # Enable/disable time drift fix.
    tdf: Optional[bool] = Field(default=None)
    # Enable/disable Template.
    template: Optional[bool] = Field(default=None)
    # Configure a Disk for storing TPM state. The format is fixed to 'raw'. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and 4 MiB will be used instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    tpmstate0: Optional[str] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(default=None)
    # Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14).
    usbs: dict[int, Optional[str]] = Field(default=None)
    # Number of hotplugged vcpus.
    vcpus: Optional[int] = Field(default=None)
    # Configure the VGA hardware.
    vga: Optional[str] = Field(default=None)
    # Use volume as VIRTIO hard disk (n is 0 to 15). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume.
    virtios: dict[int, Optional[str]] = Field(default=None)
    # Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly.
    vmgenid: Optional[str] = Field(default=None)
    # Default storage for VM state volumes/files.
    vmstatestorage: Optional[str] = Field(default=None)
    # Create a virtual hardware watchdog device.
    watchdog: Optional[str] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="hostpcis", prefix="hostpci")
        data = serialize_repeated_with_prefix(data, group="ides", prefix="ide")
        data = serialize_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = serialize_repeated_with_prefix(data, group="nets", prefix="net")
        data = serialize_repeated_with_prefix(data, group="numas", prefix="numa")
        data = serialize_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = serialize_repeated_with_prefix(data, group="satas", prefix="sata")
        data = serialize_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = serialize_repeated_with_prefix(data, group="serials", prefix="serial")
        data = serialize_repeated_with_prefix(data, group="unuseds", prefix="unused")
        data = serialize_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = serialize_repeated_with_prefix(data, group="virtios", prefix="virtio")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(
            data, group="hostpcis", prefix="hostpci"
        )
        data = data = extract_repeated_with_prefix(data, group="ides", prefix="ide")
        data = data = extract_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = data = extract_repeated_with_prefix(data, group="nets", prefix="net")
        data = data = extract_repeated_with_prefix(data, group="numas", prefix="numa")
        data = data = extract_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = data = extract_repeated_with_prefix(data, group="satas", prefix="sata")
        data = data = extract_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = data = extract_repeated_with_prefix(
            data, group="serials", prefix="serial"
        )
        data = data = extract_repeated_with_prefix(
            data, group="unuseds", prefix="unused"
        )
        data = data = extract_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = data = extract_repeated_with_prefix(
            data, group="virtios", prefix="virtio"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    """
    The VM configuration.
    """

    # Enable/disable ACPI.
    acpi: Optional[bool] = Field(default=None)
    # List of host cores used to execute guest processes, for example: 0,5,8-11
    affinity: Optional[str] = Field(default=None)
    # Enable/disable communication with the QEMU Guest Agent and its properties.
    agent: Optional[str] = Field(default=None)
    # Virtual processor architecture. Defaults to the host.
    arch: Optional[str] = Field(default=None)
    # Arbitrary arguments passed to kvm.
    args: Optional[str] = Field(default=None)
    # Configure a audio device, useful in combination with QXL/Spice.
    audio0: Optional[str] = Field(default=None)
    # Automatic restart after crash (currently ignored).
    autostart: Optional[bool] = Field(default=None)
    # Amount of target RAM for the VM in MiB. Using zero disables the ballon driver.
    balloon: Optional[int] = Field(default=None)
    # Select BIOS implementation.
    bios: Optional[str] = Field(default=None)
    # Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated.
    boot: Optional[str] = Field(default=None)
    # Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead.
    bootdisk: Optional[str] = Field(default=None)
    # This is an alias for option -ide2
    cdrom: Optional[str] = Field(default=None)
    # cloud-init: Specify custom files to replace the automatically generated ones at start.
    cicustom: Optional[str] = Field(default=None)
    # cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords.
    cipassword: Optional[str] = Field(default=None)
    # Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows.
    citype: Optional[str] = Field(default=None)
    # cloud-init: User name to change ssh keys and password for instead of the image's configured default user.
    ciuser: Optional[str] = Field(default=None)
    # The number of cores per socket.
    cores: Optional[int] = Field(default=None)
    # Emulated CPU type.
    cpu: Optional[str] = Field(default=None)
    # Limit of CPU usage.
    cpulimit: Optional[float] = Field(default=None)
    # CPU weight for a VM, will be clamped to [1, 10000] in cgroup v2.
    cpuunits: Optional[int] = Field(default=None)
    # Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # SHA1 digest of configuration file. This can be used to prevent concurrent modifications.
    digest: str
    # Configure a disk for storing EFI vars.
    efidisk0: Optional[str] = Field(default=None)
    # Freeze CPU at startup (use 'c' monitor command to start execution).
    freeze: Optional[bool] = Field(default=None)
    # Script that will be executed during various steps in the vms lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Map host PCI devices into guest.
    hostpcis: dict[int, Optional[str]] = Field(default=None)
    # Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version >= 7.1 and ostype l26 or windows > 7.
    hotplug: Optional[str] = Field(default=None)
    # Enable/disable hugepages memory.
    hugepages: Optional[str] = Field(default=None)
    # Use volume as IDE hard disk or CD-ROM (n is 0 to 3).
    ides: dict[int, Optional[str]] = Field(default=None)
    # cloud-init: Specify IP addresses and gateways for the corresponding interface.  IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified.  The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit gateway should be provided. For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires cloud-init 19.4 or newer.  If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using dhcp on IPv4.
    ipconfigs: dict[int, Optional[str]] = Field(default=None)
    # Inter-VM shared memory. Useful for direct communication between VMs, or to the host.
    ivshmem: Optional[str] = Field(default=None)
    # Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts.
    keephugepages: Optional[bool] = Field(default=None)
    # Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS.
    keyboard: Optional[str] = Field(default=None)
    # Enable/disable KVM hardware virtualization.
    kvm: Optional[bool] = Field(default=None)
    # Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS.
    localtime: Optional[bool] = Field(default=None)
    # Lock/unlock the VM.
    lock: Optional[str] = Field(default=None)
    # Specifies the QEMU machine type.
    machine: Optional[str] = Field(default=None)
    # Amount of RAM for the VM in MiB. This is the maximum available memory when you use the balloon device.
    memory: Optional[int] = Field(default=None)
    # Set maximum tolerated downtime (in seconds) for migrations.
    migrate_downtime: Optional[float] = Field(default=None)
    # Set maximum speed (in MB/s) for migrations. Value 0 is no limit.
    migrate_speed: Optional[int] = Field(default=None)
    # Set a name for the VM. Only used on the configuration web interface.
    name: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    nameserver: Optional[str] = Field(default=None)
    # Specify network devices.
    nets: dict[int, Optional[str]] = Field(default=None)
    # Enable/disable NUMA.
    numa: Optional[bool] = Field(default=None)
    # NUMA topology.
    numas: dict[int, Optional[str]] = Field(default=None)
    # Specifies whether a VM will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # Specify guest operating system.
    ostype: Optional[str] = Field(default=None)
    # Map host parallel devices (n is 0 to 2).
    parallels: dict[int, Optional[str]] = Field(default=None)
    # Sets the protection flag of the VM. This will disable the remove VM and remove disk operations.
    protection: Optional[bool] = Field(default=None)
    # Allow reboot. If set to '0' the VM exit on reboot.
    reboot: Optional[bool] = Field(default=None)
    # Configure a VirtIO-based Random Number Generator.
    rng0: Optional[str] = Field(default=None)
    # Use volume as SATA hard disk or CD-ROM (n is 0 to 5).
    satas: dict[int, Optional[str]] = Field(default=None)
    # Use volume as SCSI hard disk or CD-ROM (n is 0 to 30).
    scsis: dict[int, Optional[str]] = Field(default=None)
    # SCSI controller model
    scsihw: Optional[str] = Field(default=None)
    # cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set.
    searchdomain: Optional[str] = Field(default=None)
    # Create a serial device inside the VM (n is 0 to 3)
    serials: dict[int, Optional[str]] = Field(default=None)
    # Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd.
    shares: Optional[int] = Field(default=None)
    # Specify SMBIOS type 1 fields.
    smbios1: Optional[str] = Field(default=None)
    # The number of CPUs. Please use option -sockets instead.
    smp: Optional[int] = Field(default=None)
    # The number of CPU sockets.
    sockets: Optional[int] = Field(default=None)
    # Configure additional enhancements for SPICE.
    spice_enhancements: Optional[str] = Field(default=None)
    # cloud-init: Setup public SSH keys (one key per line, OpenSSH format).
    sshkeys: Optional[str] = Field(default=None)
    # Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'.
    startdate: Optional[str] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
    # Enable/disable the USB tablet device.
    tablet: Optional[bool] = Field(default=None)
    # Tags of the VM. This is only meta information.
    tags: Optional[str] = Field(default=None)
    # Enable/disable time drift fix.
    tdf: Optional[bool] = Field(default=None)
    # Enable/disable Template.
    template: Optional[bool] = Field(default=None)
    # Configure a Disk for storing TPM state. The format is fixed to 'raw'.
    tpmstate0: Optional[str] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(default=None)
    # Configure an USB device (n is 0 to 4, for machine version >= 7.1 and ostype l26 or windows > 7, n can be up to 14).
    usbs: dict[int, Optional[str]] = Field(default=None)
    # Number of hotplugged vcpus.
    vcpus: Optional[int] = Field(default=None)
    # Configure the VGA hardware.
    vga: Optional[str] = Field(default=None)
    # Use volume as VIRTIO hard disk (n is 0 to 15).
    virtios: dict[int, Optional[str]] = Field(default=None)
    # Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly.
    vmgenid: Optional[str] = Field(default=None)
    # Default storage for VM state volumes/files.
    vmstatestorage: Optional[str] = Field(default=None)
    # Create a virtual hardware watchdog device.
    watchdog: Optional[str] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="hostpcis", prefix="hostpci")
        data = serialize_repeated_with_prefix(data, group="ides", prefix="ide")
        data = serialize_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = serialize_repeated_with_prefix(data, group="nets", prefix="net")
        data = serialize_repeated_with_prefix(data, group="numas", prefix="numa")
        data = serialize_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = serialize_repeated_with_prefix(data, group="satas", prefix="sata")
        data = serialize_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = serialize_repeated_with_prefix(data, group="serials", prefix="serial")
        data = serialize_repeated_with_prefix(data, group="unuseds", prefix="unused")
        data = serialize_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = serialize_repeated_with_prefix(data, group="virtios", prefix="virtio")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(
            data, group="hostpcis", prefix="hostpci"
        )
        data = data = extract_repeated_with_prefix(data, group="ides", prefix="ide")
        data = data = extract_repeated_with_prefix(
            data, group="ipconfigs", prefix="ipconfig"
        )
        data = data = extract_repeated_with_prefix(data, group="nets", prefix="net")
        data = data = extract_repeated_with_prefix(data, group="numas", prefix="numa")
        data = data = extract_repeated_with_prefix(
            data, group="parallels", prefix="parallel"
        )
        data = data = extract_repeated_with_prefix(data, group="satas", prefix="sata")
        data = data = extract_repeated_with_prefix(data, group="scsis", prefix="scsi")
        data = data = extract_repeated_with_prefix(
            data, group="serials", prefix="serial"
        )
        data = data = extract_repeated_with_prefix(
            data, group="unuseds", prefix="unused"
        )
        data = data = extract_repeated_with_prefix(data, group="usbs", prefix="usb")
        data = data = extract_repeated_with_prefix(
            data, group="virtios", prefix="virtio"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Get current values (instead of pending values).
    current: Optional[bool] = Field(default=None)
    # Fetch config values from given snapshot.
    snapshot: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ConfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the virtual machine configuration with pending configuration changes applied. Set the 'current' parameter to get the current configuration instead.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> Optional[str]:
        """
        Set virtual machine options (asynchrounous API).
        """
        return self.client.post(self.path, parameters, parse_as=Optional[str])

    def put(self, parameters: PutParameters):
        """
        Set virtual machine options (synchrounous API) - You should consider using the POST method instead for any actions involving hotplug or storage allocation.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncConfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the virtual machine configuration with pending configuration changes applied. Set the 'current' parameter to get the current configuration instead.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> Optional[str]:
        """
        Set virtual machine options (asynchrounous API).
        """
        return await self.client.post(self.path, parameters, parse_as=Optional[str])

    async def put(self, parameters: PutParameters):
        """
        Set virtual machine options (synchrounous API) - You should consider using the POST method instead for any actions involving hotplug or storage allocation.
        """
        return await self.client.put(self.path, parameters)
