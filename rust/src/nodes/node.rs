pub mod aplinfo;
pub mod apt;
pub mod capabilities;
pub mod ceph;
pub mod certificates;
pub mod config;
pub mod disks;
pub mod dns;
pub mod execute;
pub mod firewall;
pub mod hardware;
pub mod hosts;
pub mod journal;
pub mod lxc;
pub mod migrateall;
pub mod netstat;
pub mod network;
pub mod qemu;
pub mod query_url_metadata;
pub mod replication;
pub mod report;
pub mod rrd;
pub mod rrddata;
pub mod scan;
pub mod sdn;
pub mod services;
pub mod spiceshell;
pub mod startall;
pub mod status;
pub mod stopall;
pub mod storage;
pub mod subscription;
pub mod suspendall;
pub mod syslog;
pub mod tasks;
pub mod termproxy;
pub mod time;
pub mod version;
pub mod vncshell;
pub mod vncwebsocket;
pub mod vzdump;
pub mod wakeonlan;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct NodeClient<T> {
    client: T,
    path: String,
}

impl<T> NodeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }

    pub fn qemu(&self) -> qemu::QemuClient<T> {
        qemu::QemuClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lxc(&self) -> lxc::LxcClient<T> {
        lxc::LxcClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ceph(&self) -> ceph::CephClient<T> {
        ceph::CephClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vzdump(&self) -> vzdump::VzdumpClient<T> {
        vzdump::VzdumpClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn services(&self) -> services::ServicesClient<T> {
        services::ServicesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn subscription(&self) -> subscription::SubscriptionClient<T> {
        subscription::SubscriptionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn network(&self) -> network::NetworkClient<T> {
        network::NetworkClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tasks(&self) -> tasks::TasksClient<T> {
        tasks::TasksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn scan(&self) -> scan::ScanClient<T> {
        scan::ScanClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn hardware(&self) -> hardware::HardwareClient<T> {
        hardware::HardwareClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn capabilities(&self) -> capabilities::CapabilitiesClient<T> {
        capabilities::CapabilitiesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn storage(&self) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn disks(&self) -> disks::DisksClient<T> {
        disks::DisksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn apt(&self) -> apt::AptClient<T> {
        apt::AptClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn replication(&self) -> replication::ReplicationClient<T> {
        replication::ReplicationClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn certificates(&self) -> certificates::CertificatesClient<T> {
        certificates::CertificatesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn sdn(&self) -> sdn::SdnClient<T> {
        sdn::SdnClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn version(&self) -> version::VersionClient<T> {
        version::VersionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn netstat(&self) -> netstat::NetstatClient<T> {
        netstat::NetstatClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn execute(&self) -> execute::ExecuteClient<T> {
        execute::ExecuteClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn wakeonlan(&self) -> wakeonlan::WakeonlanClient<T> {
        wakeonlan::WakeonlanClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn syslog(&self) -> syslog::SyslogClient<T> {
        syslog::SyslogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn journal(&self) -> journal::JournalClient<T> {
        journal::JournalClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncshell(&self) -> vncshell::VncshellClient<T> {
        vncshell::VncshellClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn termproxy(&self) -> termproxy::TermproxyClient<T> {
        termproxy::TermproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient<T> {
        vncwebsocket::VncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn spiceshell(&self) -> spiceshell::SpiceshellClient<T> {
        spiceshell::SpiceshellClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn dns(&self) -> dns::DnsClient<T> {
        dns::DnsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn time(&self) -> time::TimeClient<T> {
        time::TimeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn aplinfo(&self) -> aplinfo::AplinfoClient<T> {
        aplinfo::AplinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn query_url_metadata(&self) -> query_url_metadata::QueryUrlMetadataClient<T> {
        query_url_metadata::QueryUrlMetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn report(&self) -> report::ReportClient<T> {
        report::ReportClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn startall(&self) -> startall::StartallClient<T> {
        startall::StartallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stopall(&self) -> stopall::StopallClient<T> {
        stopall::StopallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspendall(&self) -> suspendall::SuspendallClient<T> {
        suspendall::SuspendallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn migrateall(&self) -> migrateall::MigrateallClient<T> {
        migrateall::MigrateallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn hosts(&self) -> hosts::HostsClient<T> {
        hosts::HostsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Node index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNodeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNodeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }

    pub fn qemu(&self) -> qemu::AsyncQemuClient<T> {
        qemu::AsyncQemuClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lxc(&self) -> lxc::AsyncLxcClient<T> {
        lxc::AsyncLxcClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ceph(&self) -> ceph::AsyncCephClient<T> {
        ceph::AsyncCephClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vzdump(&self) -> vzdump::AsyncVzdumpClient<T> {
        vzdump::AsyncVzdumpClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn services(&self) -> services::AsyncServicesClient<T> {
        services::AsyncServicesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn subscription(&self) -> subscription::AsyncSubscriptionClient<T> {
        subscription::AsyncSubscriptionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn network(&self) -> network::AsyncNetworkClient<T> {
        network::AsyncNetworkClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tasks(&self) -> tasks::AsyncTasksClient<T> {
        tasks::AsyncTasksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn scan(&self) -> scan::AsyncScanClient<T> {
        scan::AsyncScanClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn hardware(&self) -> hardware::AsyncHardwareClient<T> {
        hardware::AsyncHardwareClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn capabilities(&self) -> capabilities::AsyncCapabilitiesClient<T> {
        capabilities::AsyncCapabilitiesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn storage(&self) -> storage::AsyncStorageClient<T> {
        storage::AsyncStorageClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn disks(&self) -> disks::AsyncDisksClient<T> {
        disks::AsyncDisksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn apt(&self) -> apt::AsyncAptClient<T> {
        apt::AsyncAptClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn firewall(&self) -> firewall::AsyncFirewallClient<T> {
        firewall::AsyncFirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn replication(&self) -> replication::AsyncReplicationClient<T> {
        replication::AsyncReplicationClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn certificates(&self) -> certificates::AsyncCertificatesClient<T> {
        certificates::AsyncCertificatesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::AsyncConfigClient<T> {
        config::AsyncConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn sdn(&self) -> sdn::AsyncSdnClient<T> {
        sdn::AsyncSdnClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn version(&self) -> version::AsyncVersionClient<T> {
        version::AsyncVersionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn netstat(&self) -> netstat::AsyncNetstatClient<T> {
        netstat::AsyncNetstatClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn execute(&self) -> execute::AsyncExecuteClient<T> {
        execute::AsyncExecuteClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn wakeonlan(&self) -> wakeonlan::AsyncWakeonlanClient<T> {
        wakeonlan::AsyncWakeonlanClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::AsyncRrdClient<T> {
        rrd::AsyncRrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::AsyncRrddataClient<T> {
        rrddata::AsyncRrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn syslog(&self) -> syslog::AsyncSyslogClient<T> {
        syslog::AsyncSyslogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn journal(&self) -> journal::AsyncJournalClient<T> {
        journal::AsyncJournalClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncshell(&self) -> vncshell::AsyncVncshellClient<T> {
        vncshell::AsyncVncshellClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn termproxy(&self) -> termproxy::AsyncTermproxyClient<T> {
        termproxy::AsyncTermproxyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn vncwebsocket(&self) -> vncwebsocket::AsyncVncwebsocketClient<T> {
        vncwebsocket::AsyncVncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn spiceshell(&self) -> spiceshell::AsyncSpiceshellClient<T> {
        spiceshell::AsyncSpiceshellClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn dns(&self) -> dns::AsyncDnsClient<T> {
        dns::AsyncDnsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn time(&self) -> time::AsyncTimeClient<T> {
        time::AsyncTimeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn aplinfo(&self) -> aplinfo::AsyncAplinfoClient<T> {
        aplinfo::AsyncAplinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn query_url_metadata(&self) -> query_url_metadata::AsyncQueryUrlMetadataClient<T> {
        query_url_metadata::AsyncQueryUrlMetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn report(&self) -> report::AsyncReportClient<T> {
        report::AsyncReportClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn startall(&self) -> startall::AsyncStartallClient<T> {
        startall::AsyncStartallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stopall(&self) -> stopall::AsyncStopallClient<T> {
        stopall::AsyncStopallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspendall(&self) -> suspendall::AsyncSuspendallClient<T> {
        suspendall::AsyncSuspendallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn migrateall(&self) -> migrateall::AsyncMigrateallClient<T> {
        migrateall::AsyncMigrateallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn hosts(&self) -> hosts::AsyncHostsClient<T> {
        hosts::AsyncHostsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncNodeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Node index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
