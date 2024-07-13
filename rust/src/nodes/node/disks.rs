pub mod directory;
pub mod initgpt;
pub mod list;
pub mod lvm;
pub mod lvmthin;
pub mod smart;
pub mod wipedisk;
pub mod zfs;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct DisksClient<T> {
    client: T,
    path: String,
}

impl<T> DisksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "disks"),
        }
    }

    pub fn lvm(&self) -> lvm::LvmClient<T> {
        lvm::LvmClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvmthin(&self) -> lvmthin::LvmthinClient<T> {
        lvmthin::LvmthinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn directory(&self) -> directory::DirectoryClient<T> {
        directory::DirectoryClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zfs(&self) -> zfs::ZfsClient<T> {
        zfs::ZfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn list(&self) -> list::ListClient<T> {
        list::ListClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn smart(&self) -> smart::SmartClient<T> {
        smart::SmartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn initgpt(&self) -> initgpt::InitgptClient<T> {
        initgpt::InitgptClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn wipedisk(&self) -> wipedisk::WipediskClient<T> {
        wipedisk::WipediskClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Node index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDisksClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDisksClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "disks"),
        }
    }

    pub fn lvm(&self) -> lvm::AsyncLvmClient<T> {
        lvm::AsyncLvmClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lvmthin(&self) -> lvmthin::AsyncLvmthinClient<T> {
        lvmthin::AsyncLvmthinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn directory(&self) -> directory::AsyncDirectoryClient<T> {
        directory::AsyncDirectoryClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zfs(&self) -> zfs::AsyncZfsClient<T> {
        zfs::AsyncZfsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn list(&self) -> list::AsyncListClient<T> {
        list::AsyncListClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn smart(&self) -> smart::AsyncSmartClient<T> {
        smart::AsyncSmartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn initgpt(&self) -> initgpt::AsyncInitgptClient<T> {
        initgpt::AsyncInitgptClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn wipedisk(&self) -> wipedisk::AsyncWipediskClient<T> {
        wipedisk::AsyncWipediskClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncDisksClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Node index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
