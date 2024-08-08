#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "Restart pveproxy."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub restart: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "PEM encoded certificate (chain)."]
    pub certificates: String,
    #[doc = "Overwrite existing custom or ACME certificate files."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "PEM encoded private key."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    #[doc = "Restart pveproxy."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub restart: Option<bool>,
}

impl PostParameters {
    pub fn new(certificates: String) -> Self {
        Self {
            certificates,
            force: Default::default(),
            key: Default::default(),
            restart: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {
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
pub struct CustomClient<T> {
    client: T,
    path: String,
}

impl<T> CustomClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "custom"),
        }
    }
}
impl<T> CustomClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "DELETE custom certificate chain and key."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Upload or update custom certificate chain and key."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCustomClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCustomClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "custom"),
        }
    }
}
impl<T> AsyncCustomClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "DELETE custom certificate chain and key."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<(), T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Upload or update custom certificate chain and key."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
