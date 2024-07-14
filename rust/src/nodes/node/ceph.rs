pub mod cfg;
pub mod cmd_safety;
pub mod config;
pub mod configdb;
pub mod crush;
pub mod fs;
pub mod init;
pub mod log;
pub mod mds;
pub mod mgr;
pub mod mon;
pub mod osd;
pub mod pool;
pub mod pools;
pub mod restart;
pub mod rules;
pub mod start;
pub mod status;
pub mod stop;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct CephClient<T> {
    client: T,
    path: String,
}

impl<T> CephClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ceph"),
        }
    }

    pub fn cfg(&self) -> cfg::CfgClient<T> {
        cfg::CfgClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn osd(&self) -> osd::OsdClient<T> {
        osd::OsdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mds(&self) -> mds::MdsClient<T> {
        mds::MdsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mgr(&self) -> mgr::MgrClient<T> {
        mgr::MgrClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mon(&self) -> mon::MonClient<T> {
        mon::MonClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fs(&self) -> fs::FsClient<T> {
        fs::FsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pool(&self) -> pool::PoolClient<T> {
        pool::PoolClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pools(&self) -> pools::PoolsClient<T> {
        pools::PoolsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn configdb(&self) -> configdb::ConfigdbClient<T> {
        configdb::ConfigdbClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn init(&self) -> init::InitClient<T> {
        init::InitClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn restart(&self) -> restart::RestartClient<T> {
        restart::RestartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn crush(&self) -> crush::CrushClient<T> {
        crush::CrushClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rules(&self) -> rules::RulesClient<T> {
        rules::RulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cmd_safety(&self) -> cmd_safety::CmdSafetyClient<T> {
        cmd_safety::CmdSafetyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCephClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCephClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ceph"),
        }
    }

    pub fn cfg(&self) -> cfg::AsyncCfgClient<T> {
        cfg::AsyncCfgClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn osd(&self) -> osd::AsyncOsdClient<T> {
        osd::AsyncOsdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mds(&self) -> mds::AsyncMdsClient<T> {
        mds::AsyncMdsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mgr(&self) -> mgr::AsyncMgrClient<T> {
        mgr::AsyncMgrClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn mon(&self) -> mon::AsyncMonClient<T> {
        mon::AsyncMonClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fs(&self) -> fs::AsyncFsClient<T> {
        fs::AsyncFsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pool(&self) -> pool::AsyncPoolClient<T> {
        pool::AsyncPoolClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pools(&self) -> pools::AsyncPoolsClient<T> {
        pools::AsyncPoolsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn config(&self) -> config::AsyncConfigClient<T> {
        config::AsyncConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn configdb(&self) -> configdb::AsyncConfigdbClient<T> {
        configdb::AsyncConfigdbClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn init(&self) -> init::AsyncInitClient<T> {
        init::AsyncInitClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::AsyncStopClient<T> {
        stop::AsyncStopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn start(&self) -> start::AsyncStartClient<T> {
        start::AsyncStartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn restart(&self) -> restart::AsyncRestartClient<T> {
        restart::AsyncRestartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn crush(&self) -> crush::AsyncCrushClient<T> {
        crush::AsyncCrushClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::AsyncLogClient<T> {
        log::AsyncLogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rules(&self) -> rules::AsyncRulesClient<T> {
        rules::AsyncRulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cmd_safety(&self) -> cmd_safety::AsyncCmdSafetyClient<T> {
        cmd_safety::AsyncCmdSafetyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCephClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
