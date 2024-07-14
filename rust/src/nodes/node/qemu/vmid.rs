pub mod agent;
pub mod clone;
pub mod cloudinit;
pub mod config;
pub mod feature;
pub mod firewall;
pub mod migrate;
pub mod monitor;
pub mod move_disk;
pub mod mtunnel;
pub mod mtunnelwebsocket;
pub mod pending;
pub mod remote_migrate;
pub mod resize;
pub mod rrd;
pub mod rrddata;
pub mod sendkey;
pub mod snapshot;
pub mod spiceproxy;
pub mod status;
pub mod template;
pub mod termproxy;
pub mod unlink;
pub mod vncproxy;
pub mod vncwebsocket;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "If set, destroy additionally all disks not referenced in the config but with a matching VMID from all enabled storages."]
    #[serde(
        rename = "destroy-unreferenced-disks",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub destroy_unreferenced_disks: Option<bool>,
    #[doc = "Remove VMID from configurations, like backup & replication jobs and HA."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub purge: Option<bool>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct VmidClient<T> {
    client: T,
    path: String,
}

impl<T> VmidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, vmid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vmid),
        }
    }

    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn agent(&self) -> agent::AgentClient<T> {
        agent::AgentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pending(&self) -> pending::PendingClient<T> {
        pending::PendingClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cloudinit(&self) -> cloudinit::CloudinitClient<T> {
        cloudinit::CloudinitClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn unlink(&self) -> unlink::UnlinkClient<T> {
        unlink::UnlinkClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncproxy(&self) -> vncproxy::VncproxyClient<T> {
        vncproxy::VncproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn termproxy(&self) -> termproxy::TermproxyClient<T> {
        termproxy::TermproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient<T> {
        vncwebsocket::VncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn spiceproxy(&self) -> spiceproxy::SpiceproxyClient<T> {
        spiceproxy::SpiceproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn sendkey(&self) -> sendkey::SendkeyClient<T> {
        sendkey::SendkeyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn feature(&self) -> feature::FeatureClient<T> {
        feature::FeatureClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn clone(&self) -> clone::CloneClient<T> {
        clone::CloneClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn move_disk(&self) -> move_disk::MoveDiskClient<T> {
        move_disk::MoveDiskClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn remote_migrate(&self) -> remote_migrate::RemoteMigrateClient<T> {
        remote_migrate::RemoteMigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn monitor(&self) -> monitor::MonitorClient<T> {
        monitor::MonitorClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resize(&self) -> resize::ResizeClient<T> {
        resize::ResizeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn snapshot(&self) -> snapshot::SnapshotClient<T> {
        snapshot::SnapshotClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn template(&self) -> template::TemplateClient<T> {
        template::TemplateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mtunnel(&self) -> mtunnel::MtunnelClient<T> {
        mtunnel::MtunnelClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mtunnelwebsocket(&self) -> mtunnelwebsocket::MtunnelwebsocketClient<T> {
        mtunnelwebsocket::MtunnelwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy the VM and  all used/owned volumes. Removes any VM specific permissions and firewall rules"]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Directory index"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVmidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVmidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, vmid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vmid),
        }
    }

    pub fn firewall(&self) -> firewall::AsyncFirewallClient<T> {
        firewall::AsyncFirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn agent(&self) -> agent::AsyncAgentClient<T> {
        agent::AsyncAgentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::AsyncRrdClient<T> {
        rrd::AsyncRrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::AsyncRrddataClient<T> {
        rrddata::AsyncRrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::AsyncConfigClient<T> {
        config::AsyncConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pending(&self) -> pending::AsyncPendingClient<T> {
        pending::AsyncPendingClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cloudinit(&self) -> cloudinit::AsyncCloudinitClient<T> {
        cloudinit::AsyncCloudinitClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn unlink(&self) -> unlink::AsyncUnlinkClient<T> {
        unlink::AsyncUnlinkClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncproxy(&self) -> vncproxy::AsyncVncproxyClient<T> {
        vncproxy::AsyncVncproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn termproxy(&self) -> termproxy::AsyncTermproxyClient<T> {
        termproxy::AsyncTermproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncwebsocket(&self) -> vncwebsocket::AsyncVncwebsocketClient<T> {
        vncwebsocket::AsyncVncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn spiceproxy(&self) -> spiceproxy::AsyncSpiceproxyClient<T> {
        spiceproxy::AsyncSpiceproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn sendkey(&self) -> sendkey::AsyncSendkeyClient<T> {
        sendkey::AsyncSendkeyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn feature(&self) -> feature::AsyncFeatureClient<T> {
        feature::AsyncFeatureClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn clone(&self) -> clone::AsyncCloneClient<T> {
        clone::AsyncCloneClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn move_disk(&self) -> move_disk::AsyncMoveDiskClient<T> {
        move_disk::AsyncMoveDiskClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn migrate(&self) -> migrate::AsyncMigrateClient<T> {
        migrate::AsyncMigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn remote_migrate(&self) -> remote_migrate::AsyncRemoteMigrateClient<T> {
        remote_migrate::AsyncRemoteMigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn monitor(&self) -> monitor::AsyncMonitorClient<T> {
        monitor::AsyncMonitorClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resize(&self) -> resize::AsyncResizeClient<T> {
        resize::AsyncResizeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn snapshot(&self) -> snapshot::AsyncSnapshotClient<T> {
        snapshot::AsyncSnapshotClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn template(&self) -> template::AsyncTemplateClient<T> {
        template::AsyncTemplateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mtunnel(&self) -> mtunnel::AsyncMtunnelClient<T> {
        mtunnel::AsyncMtunnelClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mtunnelwebsocket(&self) -> mtunnelwebsocket::AsyncMtunnelwebsocketClient<T> {
        mtunnelwebsocket::AsyncMtunnelwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncVmidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy the VM and  all used/owned volumes. Removes any VM specific permissions and firewall rules"]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Directory index"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
