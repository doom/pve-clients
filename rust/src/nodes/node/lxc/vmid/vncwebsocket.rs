#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Port number returned by previous vncproxy call."]
    pub port: u64,
    #[doc = "Ticket from previous call to vncproxy."]
    pub vncticket: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct VncwebsocketClient<T> {
    client: T,
    path: String,
}

impl<T> VncwebsocketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncwebsocket"),
        }
    }
}
impl<T> VncwebsocketClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Opens a weksocket for VNC traffic."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVncwebsocketClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVncwebsocketClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "vncwebsocket"),
        }
    }
}
impl<T> AsyncVncwebsocketClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Opens a weksocket for VNC traffic."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
