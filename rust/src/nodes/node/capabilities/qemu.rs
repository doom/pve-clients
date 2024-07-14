pub mod cpu;
pub mod machines;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct QemuClient<T> {
    client: T,
    path: String,
}

impl<T> QemuClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "qemu"),
        }
    }

    pub fn cpu(&self) -> cpu::CpuClient<T> {
        cpu::CpuClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn machines(&self) -> machines::MachinesClient<T> {
        machines::MachinesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> QemuClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "QEMU capabilities index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncQemuClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncQemuClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "qemu"),
        }
    }

    pub fn cpu(&self) -> cpu::AsyncCpuClient<T> {
        cpu::AsyncCpuClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn machines(&self) -> machines::AsyncMachinesClient<T> {
        machines::AsyncMachinesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncQemuClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "QEMU capabilities index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
