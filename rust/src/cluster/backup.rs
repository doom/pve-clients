pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The job ID."]
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Backup all known guest systems on this host."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub all: Option<bool>,
    #[doc = "Limit I/O bandwidth (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<u64>,
    #[doc = "Description for the Job."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Compress dump file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub compress: Option<String>,
    #[doc = "Day of week selection."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dow: Option<String>,
    #[doc = "Store resulting files to specified directory."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dumpdir: Option<String>,
    #[doc = "Enable or disable the job."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Exclude specified guest systems (assumes --all)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exclude: Option<String>,
    #[doc = "Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root, other paths match relative to each subdirectory."]
    #[serde(
        rename = "exclude-path",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exclude_path: Option<Vec<String>>,
    #[doc = "Options for backup fleecing (VM only)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fleecing: Option<String>,
    #[doc = "Job ID (will be autogenerated)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    #[doc = "Set IO priority when using the BFQ scheduler. For snapshot and suspend mode backups of VMs, this only affects the compressor. A value of 8 means the idle priority is used, otherwise the best-effort priority is used with the specified value."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ionice: Option<u64>,
    #[doc = "Maximal time to wait for the global lock (minutes)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lockwait: Option<u64>,
    #[doc = "Deprecated: use notification targets/matchers instead. Specify when to send a notification mail"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mailnotification: Option<String>,
    #[doc = "Deprecated: Use notification targets/matchers instead. Comma-separated list of email addresses or users that should receive email notifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mailto: Option<String>,
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxfiles: Option<u64>,
    #[doc = "Backup mode."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "Only run if executed on this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub node: Option<String>,
    #[doc = "Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\\n' and '\\\\' respectively."]
    #[serde(
        rename = "notes-template",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub notes_template: Option<String>,
    #[doc = "Determine which notification system to use. If set to 'legacy-sendmail', vzdump will consider the mailto/mailnotification parameters and send emails to the specified address(es) via the 'sendmail' command. If set to 'notification-system', a notification will be sent via PVE's notification system, and the mailto and mailnotification will be ignored. If set to 'auto' (default setting), an email will be sent if mailto is set, and the notification system will be used if not."]
    #[serde(
        rename = "notification-mode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub notification_mode: Option<String>,
    #[doc = "Deprecated: Do not use"]
    #[serde(
        rename = "notification-policy",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub notification_policy: Option<String>,
    #[doc = "Deprecated: Do not use"]
    #[serde(
        rename = "notification-target",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub notification_target: Option<String>,
    #[doc = "EXPERIMENTAL: PBS mode used to detect file changes and switch encoding format for container backups."]
    #[serde(
        rename = "pbs-change-detection-mode",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pbs_change_detection_mode: Option<String>,
    #[doc = "Other performance-related settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub performance: Option<String>,
    #[doc = "Use pigz instead of gzip when N>0. N=1 uses half of cores, N>1 uses N as thread count."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pigz: Option<u64>,
    #[doc = "Backup all known guest systems included in the specified pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pool: Option<String>,
    #[doc = "If true, mark backup(s) as protected."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub protected: Option<bool>,
    #[doc = "Use these retention options instead of those from the storage configuration."]
    #[serde(
        rename = "prune-backups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub prune_backups: Option<String>,
    #[doc = "Be quiet."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub quiet: Option<bool>,
    #[doc = "Prune older backups according to 'prune-backups'."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub remove: Option<bool>,
    #[doc = "If true, the job will be run as soon as possible if it was missed while the scheduler was not running."]
    #[serde(
        rename = "repeat-missed",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub repeat_missed: Option<bool>,
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schedule: Option<String>,
    #[doc = "Use specified hook script."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub script: Option<String>,
    #[doc = "Job Start time."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub starttime: Option<String>,
    #[doc = "Exclude temporary files and logs."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub stdexcludes: Option<bool>,
    #[doc = "Stop running backup jobs on this host."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub stop: Option<bool>,
    #[doc = "Maximal time to wait until a guest system is stopped (minutes)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stopwait: Option<u64>,
    #[doc = "Store resulting file to this storage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Store temporary files to specified directory."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tmpdir: Option<String>,
    #[doc = "The ID of the guest system you want to backup."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<String>,
    #[doc = "Zstd threads. N=0 uses half of the available cores, if N is set to a value bigger than 0, N is used as thread count."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub zstd: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BackupClient<T> {
    client: T,
    path: String,
}

impl<T> BackupClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "backup"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> BackupClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List vzdump backup schedule."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new vzdump backup job."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncBackupClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncBackupClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "backup"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncBackupClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List vzdump backup schedule."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new vzdump backup job."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
