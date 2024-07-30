#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(
        rename = "Datastore.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub datastore_allocate: Option<bool>,
    #[serde(
        rename = "Datastore.AllocateSpace",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub datastore_allocate_space: Option<bool>,
    #[serde(
        rename = "Datastore.AllocateTemplate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub datastore_allocate_template: Option<bool>,
    #[serde(
        rename = "Datastore.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub datastore_audit: Option<bool>,
    #[serde(
        rename = "Group.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub group_allocate: Option<bool>,
    #[serde(
        rename = "Mapping.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mapping_audit: Option<bool>,
    #[serde(
        rename = "Mapping.Modify",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mapping_modify: Option<bool>,
    #[serde(
        rename = "Mapping.Use",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mapping_use: Option<bool>,
    #[serde(
        rename = "Permissions.Modify",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub permissions_modify: Option<bool>,
    #[serde(
        rename = "Pool.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pool_allocate: Option<bool>,
    #[serde(
        rename = "Pool.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pool_audit: Option<bool>,
    #[serde(
        rename = "Realm.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub realm_allocate: Option<bool>,
    #[serde(
        rename = "Realm.AllocateUser",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub realm_allocate_user: Option<bool>,
    #[serde(
        rename = "SDN.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sdn_allocate: Option<bool>,
    #[serde(
        rename = "SDN.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sdn_audit: Option<bool>,
    #[serde(
        rename = "SDN.Use",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sdn_use: Option<bool>,
    #[serde(
        rename = "Sys.AccessNetwork",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_access_network: Option<bool>,
    #[serde(
        rename = "Sys.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_audit: Option<bool>,
    #[serde(
        rename = "Sys.Console",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_console: Option<bool>,
    #[serde(
        rename = "Sys.Incoming",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_incoming: Option<bool>,
    #[serde(
        rename = "Sys.Modify",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_modify: Option<bool>,
    #[serde(
        rename = "Sys.PowerMgmt",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_power_mgmt: Option<bool>,
    #[serde(
        rename = "Sys.Syslog",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sys_syslog: Option<bool>,
    #[serde(
        rename = "User.Modify",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub user_modify: Option<bool>,
    #[serde(
        rename = "VM.Allocate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_allocate: Option<bool>,
    #[serde(
        rename = "VM.Audit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_audit: Option<bool>,
    #[serde(
        rename = "VM.Backup",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_backup: Option<bool>,
    #[serde(
        rename = "VM.Clone",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_clone: Option<bool>,
    #[serde(
        rename = "VM.Config.CDROM",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_cdrom: Option<bool>,
    #[serde(
        rename = "VM.Config.CPU",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_cpu: Option<bool>,
    #[serde(
        rename = "VM.Config.Cloudinit",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_cloudinit: Option<bool>,
    #[serde(
        rename = "VM.Config.Disk",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_disk: Option<bool>,
    #[serde(
        rename = "VM.Config.HWType",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_hwtype: Option<bool>,
    #[serde(
        rename = "VM.Config.Memory",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_memory: Option<bool>,
    #[serde(
        rename = "VM.Config.Network",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_network: Option<bool>,
    #[serde(
        rename = "VM.Config.Options",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_config_options: Option<bool>,
    #[serde(
        rename = "VM.Console",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_console: Option<bool>,
    #[serde(
        rename = "VM.Migrate",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_migrate: Option<bool>,
    #[serde(
        rename = "VM.Monitor",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_monitor: Option<bool>,
    #[serde(
        rename = "VM.PowerMgmt",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_power_mgmt: Option<bool>,
    #[serde(
        rename = "VM.Snapshot",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_snapshot: Option<bool>,
    #[serde(
        rename = "VM.Snapshot.Rollback",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vm_snapshot_rollback: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RoleidClient<T> {
    client: T,
    path: String,
}

impl<T> RoleidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, roleid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, roleid),
        }
    }
}
impl<T> RoleidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete role."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get role configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update an existing role."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRoleidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRoleidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, roleid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, roleid),
        }
    }
}
impl<T> AsyncRoleidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete role."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get role configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update an existing role."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
