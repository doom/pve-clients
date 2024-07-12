pub mod certificate;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct AcmeClient<T> {
    client: T,
    path: String,
}

impl<T> AcmeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acme"),
        }
    }

    pub fn certificate(&self) -> certificate::CertificateClient<T> {
        certificate::CertificateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ACME index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAcmeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAcmeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acme"),
        }
    }

    pub fn certificate(&self) -> certificate::AsyncCertificateClient<T> {
        certificate::AsyncCertificateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncAcmeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ACME index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
