#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "SPICE proxy server. This can be used by the client to specify the proxy server. All nodes in a cluster runs 'spiceproxy', so it is up to the client to choose one. By default, we return the node where the VM is currently running. As reasonable setting is to use same node you use to connect to the API (This is window.location.hostname for the JS GUI)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proxy: Option<String>,
}

#[doc = "Returned values can be directly passed to the 'remote-viewer' application."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub host: String,
    pub password: String,
    pub proxy: String,
    #[serde(rename = "tls-port")]
    pub tls_port: u64,
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct SpiceproxyClient<T> {
    client: T,
    path: String,
}

impl<T> SpiceproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "spiceproxy"),
        }
    }
}
impl<T> SpiceproxyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a SPICE configuration to connect to the CT."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSpiceproxyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSpiceproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "spiceproxy"),
        }
    }
}
impl<T> AsyncSpiceproxyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a SPICE configuration to connect to the CT."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
