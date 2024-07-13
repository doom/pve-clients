pub mod acme;
pub mod custom;
pub mod info;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct CertificatesClient<T> {
    client: T,
    path: String,
}

impl<T> CertificatesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "certificates"),
        }
    }

    pub fn acme(&self) -> acme::AcmeClient<T> {
        acme::AcmeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn info(&self) -> info::InfoClient<T> {
        info::InfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn custom(&self) -> custom::CustomClient<T> {
        custom::CustomClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CertificatesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Node index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCertificatesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCertificatesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "certificates"),
        }
    }

    pub fn acme(&self) -> acme::AsyncAcmeClient<T> {
        acme::AsyncAcmeClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn info(&self) -> info::AsyncInfoClient<T> {
        info::AsyncInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn custom(&self) -> custom::AsyncCustomClient<T> {
        custom::AsyncCustomClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCertificatesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Node index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
