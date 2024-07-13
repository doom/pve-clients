pub mod acme;
pub mod backup;
pub mod backup_info;
pub mod ceph;
pub mod config;
pub mod firewall;
pub mod ha;
pub mod jobs;
pub mod log;
pub mod metrics;
pub mod nextid;
pub mod options;
pub mod replication;
pub mod resources;
pub mod status;
pub mod tasks;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct ClusterClient<T> {
    client: T,
    path: String,
}

impl<T> ClusterClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "cluster".to_string(),
        }
    }

    pub fn replication(&self) -> replication::ReplicationClient<T> {
        replication::ReplicationClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn metrics(&self) -> metrics::MetricsClient<T> {
        metrics::MetricsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn backup(&self) -> backup::BackupClient<T> {
        backup::BackupClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn backup_info(&self) -> backup_info::BackupInfoClient<T> {
        backup_info::BackupInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ha(&self) -> ha::HaClient<T> {
        ha::HaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn acme(&self) -> acme::AcmeClient<T> {
        acme::AcmeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ceph(&self) -> ceph::CephClient<T> {
        ceph::CephClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn jobs(&self) -> jobs::JobsClient<T> {
        jobs::JobsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resources(&self) -> resources::ResourcesClient<T> {
        resources::ResourcesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tasks(&self) -> tasks::TasksClient<T> {
        tasks::TasksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::OptionsClient<T> {
        options::OptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn nextid(&self) -> nextid::NextidClient<T> {
        nextid::NextidClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Cluster index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncClusterClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncClusterClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "cluster".to_string(),
        }
    }

    pub fn replication(&self) -> replication::AsyncReplicationClient<T> {
        replication::AsyncReplicationClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn metrics(&self) -> metrics::AsyncMetricsClient<T> {
        metrics::AsyncMetricsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::AsyncConfigClient<T> {
        config::AsyncConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn firewall(&self) -> firewall::AsyncFirewallClient<T> {
        firewall::AsyncFirewallClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn backup(&self) -> backup::AsyncBackupClient<T> {
        backup::AsyncBackupClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn backup_info(&self) -> backup_info::AsyncBackupInfoClient<T> {
        backup_info::AsyncBackupInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ha(&self) -> ha::AsyncHaClient<T> {
        ha::AsyncHaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn acme(&self) -> acme::AsyncAcmeClient<T> {
        acme::AsyncAcmeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ceph(&self) -> ceph::AsyncCephClient<T> {
        ceph::AsyncCephClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn jobs(&self) -> jobs::AsyncJobsClient<T> {
        jobs::AsyncJobsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::AsyncLogClient<T> {
        log::AsyncLogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resources(&self) -> resources::AsyncResourcesClient<T> {
        resources::AsyncResourcesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tasks(&self) -> tasks::AsyncTasksClient<T> {
        tasks::AsyncTasksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::AsyncOptionsClient<T> {
        options::AsyncOptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn nextid(&self) -> nextid::AsyncNextidClient<T> {
        nextid::AsyncNextidClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncClusterClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Cluster index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
