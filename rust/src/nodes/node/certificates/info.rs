#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filename: Option<String>,
    #[doc = "Certificate SHA 256 fingerprint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fingerprint: Option<String>,
    #[doc = "Certificate issuer name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub issuer: Option<String>,
    #[doc = "Certificate's notAfter timestamp (UNIX epoch)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notafter: Option<u64>,
    #[doc = "Certificate's notBefore timestamp (UNIX epoch)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub notbefore: Option<u64>,
    #[doc = "Certificate in PEM format"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pem: Option<String>,
    #[doc = "Certificate's public key size"]
    #[serde(
        rename = "public-key-bits",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_key_bits: Option<u64>,
    #[doc = "Certificate's public key algorithm"]
    #[serde(
        rename = "public-key-type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub public_key_type: Option<String>,
    #[doc = "List of Certificate's SubjectAlternativeName entries."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub san: Option<Vec<String>>,
    #[doc = "Certificate subject name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InfoClient<T> {
    client: T,
    path: String,
}

impl<T> InfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "info"),
        }
    }
}
impl<T> InfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get information about node's certificates."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "info"),
        }
    }
}
impl<T> AsyncInfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get information about node's certificates."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
