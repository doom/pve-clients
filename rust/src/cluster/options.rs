#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<String>,
    #[doc = "Select the default Console viewer. You can either use the builtin java applet (VNC; deprecated and maps to html5), an external virt-viewer comtatible application (SPICE), an HTML5 based vnc viewer (noVNC), or an HTML5 based console client (xtermjs). If the selected viewer is not available (e.g. SPICE not activated for the VM), the fallback is noVNC."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub console: Option<String>,
    #[doc = "Cluster resource scheduling settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub crs: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Datacenter description. Shown in the web-interface datacenter notes panel. This is saved as comment inside the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Specify email address to send notification from (default is root@$hostname)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email_from: Option<String>,
    #[doc = "Set the fencing mode of the HA cluster. Hardware mode needs a valid configuration of fence devices in /etc/pve/ha/fence.cfg. With both all two modes are used.  WARNING: 'hardware' and 'both' are EXPERIMENTAL & WIP"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fencing: Option<String>,
    #[doc = "Cluster wide HA settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ha: Option<String>,
    #[doc = "Specify external http proxy which is used for downloads (example: 'http://username:password@host:port/')"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub http_proxy: Option<String>,
    #[doc = "Default keybord layout for vnc server."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keyboard: Option<String>,
    #[doc = "Default GUI language."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub language: Option<String>,
    #[doc = "Prefix for the auto-generated MAC addresses of virtual guests. The default 'BC:24:11' is the OUI assigned by the IEEE to Proxmox Server Solutions GmbH for a 24-bit large MAC block. You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or behind NAT)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mac_prefix: Option<String>,
    #[doc = "Defines how many workers (per node) are maximal started  on actions like 'stopall VMs' or task from the ha-manager."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_workers: Option<u64>,
    #[doc = "For cluster wide migration settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migration: Option<String>,
    #[doc = "Migration is secure using SSH tunnel by default. For secure private networks you can disable it to speed up migration. Deprecated, use the 'migration' property instead!"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub migration_unsecure: Option<bool>,
    #[doc = "Control the range for the free VMID auto-selection pool."]
    #[serde(rename = "next-id", skip_serializing_if = "Option::is_none", default)]
    pub next_id: Option<String>,
    #[doc = "Cluster-wide notification settings."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notify: Option<String>,
    #[doc = "A list of tags that require a `Sys.Modify` on '/' to set and delete. Tags set here that are also in 'user-tag-access' also require `Sys.Modify`."]
    #[serde(
        rename = "registered-tags",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub registered_tags: Option<String>,
    #[doc = "Tag style options."]
    #[serde(rename = "tag-style", skip_serializing_if = "Option::is_none", default)]
    pub tag_style: Option<String>,
    #[doc = "u2f"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub u2f: Option<String>,
    #[doc = "Privilege options for user-settable tags"]
    #[serde(
        rename = "user-tag-access",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub user_tag_access: Option<String>,
    #[doc = "webauthn configuration"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub webauthn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OptionsClient<T> {
    client: T,
    path: String,
}

impl<T> OptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set datacenter options."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOptionsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOptionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "options"),
        }
    }
}
impl<T> AsyncOptionsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set datacenter options."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
