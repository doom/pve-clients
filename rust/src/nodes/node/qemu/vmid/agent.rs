pub mod exec;
pub mod exec_status;
pub mod file_read;
pub mod file_write;
pub mod fsfreeze_freeze;
pub mod fsfreeze_status;
pub mod fsfreeze_thaw;
pub mod fstrim;
pub mod get_fsinfo;
pub mod get_host_name;
pub mod get_memory_block_info;
pub mod get_memory_blocks;
pub mod get_osinfo;
pub mod get_time;
pub mod get_timezone;
pub mod get_users;
pub mod get_vcpus;
pub mod info;
pub mod network_get_interfaces;
pub mod ping;
pub mod set_user_password;
pub mod shutdown;
pub mod suspend_disk;
pub mod suspend_hybrid;
pub mod suspend_ram;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The QGA command."]
    pub command: String,
}

#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct AgentClient<T> {
    client: T,
    path: String,
}

impl<T> AgentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "agent"),
        }
    }

    pub fn fsfreeze_freeze(&self) -> fsfreeze_freeze::FsfreezeFreezeClient<T> {
        fsfreeze_freeze::FsfreezeFreezeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fsfreeze_status(&self) -> fsfreeze_status::FsfreezeStatusClient<T> {
        fsfreeze_status::FsfreezeStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fsfreeze_thaw(&self) -> fsfreeze_thaw::FsfreezeThawClient<T> {
        fsfreeze_thaw::FsfreezeThawClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fstrim(&self) -> fstrim::FstrimClient<T> {
        fstrim::FstrimClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_fsinfo(&self) -> get_fsinfo::GetFsinfoClient<T> {
        get_fsinfo::GetFsinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_host_name(&self) -> get_host_name::GetHostNameClient<T> {
        get_host_name::GetHostNameClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_memory_block_info(&self) -> get_memory_block_info::GetMemoryBlockInfoClient<T> {
        get_memory_block_info::GetMemoryBlockInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_memory_blocks(&self) -> get_memory_blocks::GetMemoryBlocksClient<T> {
        get_memory_blocks::GetMemoryBlocksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_osinfo(&self) -> get_osinfo::GetOsinfoClient<T> {
        get_osinfo::GetOsinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_time(&self) -> get_time::GetTimeClient<T> {
        get_time::GetTimeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_timezone(&self) -> get_timezone::GetTimezoneClient<T> {
        get_timezone::GetTimezoneClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_users(&self) -> get_users::GetUsersClient<T> {
        get_users::GetUsersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_vcpus(&self) -> get_vcpus::GetVcpusClient<T> {
        get_vcpus::GetVcpusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn info(&self) -> info::InfoClient<T> {
        info::InfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn network_get_interfaces(&self) -> network_get_interfaces::NetworkGetInterfacesClient<T> {
        network_get_interfaces::NetworkGetInterfacesClient::<T>::new(
            self.client.clone(),
            &self.path,
        )
    }

    pub fn ping(&self) -> ping::PingClient<T> {
        ping::PingClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn shutdown(&self) -> shutdown::ShutdownClient<T> {
        shutdown::ShutdownClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_disk(&self) -> suspend_disk::SuspendDiskClient<T> {
        suspend_disk::SuspendDiskClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_hybrid(&self) -> suspend_hybrid::SuspendHybridClient<T> {
        suspend_hybrid::SuspendHybridClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_ram(&self) -> suspend_ram::SuspendRamClient<T> {
        suspend_ram::SuspendRamClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn set_user_password(&self) -> set_user_password::SetUserPasswordClient<T> {
        set_user_password::SetUserPasswordClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn exec(&self) -> exec::ExecClient<T> {
        exec::ExecClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn exec_status(&self) -> exec_status::ExecStatusClient<T> {
        exec_status::ExecStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_read(&self) -> file_read::FileReadClient<T> {
        file_read::FileReadClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_write(&self) -> file_write::FileWriteClient<T> {
        file_write::FileWriteClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "QEMU Guest Agent command index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Execute QEMU Guest Agent commands."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAgentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAgentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "agent"),
        }
    }

    pub fn fsfreeze_freeze(&self) -> fsfreeze_freeze::AsyncFsfreezeFreezeClient<T> {
        fsfreeze_freeze::AsyncFsfreezeFreezeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fsfreeze_status(&self) -> fsfreeze_status::AsyncFsfreezeStatusClient<T> {
        fsfreeze_status::AsyncFsfreezeStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fsfreeze_thaw(&self) -> fsfreeze_thaw::AsyncFsfreezeThawClient<T> {
        fsfreeze_thaw::AsyncFsfreezeThawClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn fstrim(&self) -> fstrim::AsyncFstrimClient<T> {
        fstrim::AsyncFstrimClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_fsinfo(&self) -> get_fsinfo::AsyncGetFsinfoClient<T> {
        get_fsinfo::AsyncGetFsinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_host_name(&self) -> get_host_name::AsyncGetHostNameClient<T> {
        get_host_name::AsyncGetHostNameClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_memory_block_info(&self) -> get_memory_block_info::AsyncGetMemoryBlockInfoClient<T> {
        get_memory_block_info::AsyncGetMemoryBlockInfoClient::<T>::new(
            self.client.clone(),
            &self.path,
        )
    }

    pub fn get_memory_blocks(&self) -> get_memory_blocks::AsyncGetMemoryBlocksClient<T> {
        get_memory_blocks::AsyncGetMemoryBlocksClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_osinfo(&self) -> get_osinfo::AsyncGetOsinfoClient<T> {
        get_osinfo::AsyncGetOsinfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_time(&self) -> get_time::AsyncGetTimeClient<T> {
        get_time::AsyncGetTimeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_timezone(&self) -> get_timezone::AsyncGetTimezoneClient<T> {
        get_timezone::AsyncGetTimezoneClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_users(&self) -> get_users::AsyncGetUsersClient<T> {
        get_users::AsyncGetUsersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn get_vcpus(&self) -> get_vcpus::AsyncGetVcpusClient<T> {
        get_vcpus::AsyncGetVcpusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn info(&self) -> info::AsyncInfoClient<T> {
        info::AsyncInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn network_get_interfaces(
        &self,
    ) -> network_get_interfaces::AsyncNetworkGetInterfacesClient<T> {
        network_get_interfaces::AsyncNetworkGetInterfacesClient::<T>::new(
            self.client.clone(),
            &self.path,
        )
    }

    pub fn ping(&self) -> ping::AsyncPingClient<T> {
        ping::AsyncPingClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn shutdown(&self) -> shutdown::AsyncShutdownClient<T> {
        shutdown::AsyncShutdownClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_disk(&self) -> suspend_disk::AsyncSuspendDiskClient<T> {
        suspend_disk::AsyncSuspendDiskClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_hybrid(&self) -> suspend_hybrid::AsyncSuspendHybridClient<T> {
        suspend_hybrid::AsyncSuspendHybridClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend_ram(&self) -> suspend_ram::AsyncSuspendRamClient<T> {
        suspend_ram::AsyncSuspendRamClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn set_user_password(&self) -> set_user_password::AsyncSetUserPasswordClient<T> {
        set_user_password::AsyncSetUserPasswordClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn exec(&self) -> exec::AsyncExecClient<T> {
        exec::AsyncExecClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn exec_status(&self) -> exec_status::AsyncExecStatusClient<T> {
        exec_status::AsyncExecStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_read(&self) -> file_read::AsyncFileReadClient<T> {
        file_read::AsyncFileReadClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_write(&self) -> file_write::AsyncFileWriteClient<T> {
        file_write::AsyncFileWriteClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncAgentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "QEMU Guest Agent command index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Execute QEMU Guest Agent commands."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
