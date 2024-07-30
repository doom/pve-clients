pub mod controllers;
pub mod dns;
pub mod ipams;
pub mod vnets;
pub mod zones;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct SdnClient<T> {
    client: T,
    path: String,
}

impl<T> SdnClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sdn"),
        }
    }

    pub fn vnets(&self) -> vnets::VnetsClient<T> {
        vnets::VnetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zones(&self) -> zones::ZonesClient<T> {
        zones::ZonesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn controllers(&self) -> controllers::ControllersClient<T> {
        controllers::ControllersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ipams(&self) -> ipams::IpamsClient<T> {
        ipams::IpamsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn dns(&self) -> dns::DnsClient<T> {
        dns::DnsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Apply sdn controller changes && reload."]
    pub fn put(&self) -> Result<String, T::Error> {
        self.client.put(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSdnClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSdnClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sdn"),
        }
    }

    pub fn vnets(&self) -> vnets::AsyncVnetsClient<T> {
        vnets::AsyncVnetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn zones(&self) -> zones::AsyncZonesClient<T> {
        zones::AsyncZonesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn controllers(&self) -> controllers::AsyncControllersClient<T> {
        controllers::AsyncControllersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ipams(&self) -> ipams::AsyncIpamsClient<T> {
        ipams::AsyncIpamsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn dns(&self) -> dns::AsyncDnsClient<T> {
        dns::AsyncDnsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncSdnClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Apply sdn controller changes && reload."]
    pub async fn put(&self) -> Result<String, T::Error> {
        self.client.put(&self.path, &()).await
    }
}
