#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Overwrite existing custom certificate."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Force renewal even if expiry is more than 30 days away."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct CertificateClient<T> {
    client: T,
    path: String,
}

impl<T> CertificateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "certificate"),
        }
    }
}
impl<T> CertificateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Revoke existing certificate from CA."]
    pub fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Order a new certificate from ACME-compatible CA."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Renew existing certificate from CA."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCertificateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCertificateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "certificate"),
        }
    }
}
impl<T> AsyncCertificateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Revoke existing certificate from CA."]
    pub async fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Order a new certificate from ACME-compatible CA."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Renew existing certificate from CA."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
