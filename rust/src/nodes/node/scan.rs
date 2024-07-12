pub mod cifs;
pub mod glusterfs;
pub mod iscsi;
pub mod lvm;
pub mod lvmthin;
pub mod nfs;
pub mod pbs;
pub mod zfs;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub method: String,
}

#[derive(Debug, Clone)]
pub struct ScanClient<T> {
    client: T,
    path: String,
}

impl<T> ScanClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "scan"),
        }
    }

    pub fn nfs(&self) -> nfs::NfsClient<T> {
        nfs::NfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cifs(&self) -> cifs::CifsClient<T> {
        cifs::CifsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pbs(&self) -> pbs::PbsClient<T> {
        pbs::PbsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn glusterfs(&self) -> glusterfs::GlusterfsClient<T> {
        glusterfs::GlusterfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn iscsi(&self) -> iscsi::IscsiClient<T> {
        iscsi::IscsiClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvm(&self) -> lvm::LvmClient<T> {
        lvm::LvmClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvmthin(&self) -> lvmthin::LvmthinClient<T> {
        lvmthin::LvmthinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zfs(&self) -> zfs::ZfsClient<T> {
        zfs::ZfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index of available scan methods"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncScanClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncScanClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "scan"),
        }
    }

    pub fn nfs(&self) -> nfs::AsyncNfsClient<T> {
        nfs::AsyncNfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn cifs(&self) -> cifs::AsyncCifsClient<T> {
        cifs::AsyncCifsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn pbs(&self) -> pbs::AsyncPbsClient<T> {
        pbs::AsyncPbsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn glusterfs(&self) -> glusterfs::AsyncGlusterfsClient<T> {
        glusterfs::AsyncGlusterfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn iscsi(&self) -> iscsi::AsyncIscsiClient<T> {
        iscsi::AsyncIscsiClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvm(&self) -> lvm::AsyncLvmClient<T> {
        lvm::AsyncLvmClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvmthin(&self) -> lvmthin::AsyncLvmthinClient<T> {
        lvmthin::AsyncLvmthinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zfs(&self) -> zfs::AsyncZfsClient<T> {
        zfs::AsyncZfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncScanClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index of available scan methods"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
