pub mod apiversion;
pub mod join;
pub mod nodes;
pub mod qdevice;
pub mod totem;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The name of the cluster."]
    pub clustername: String,
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[serde(
        default,
        flatten,
        deserialize_with = "deserialize_repeated_link_in_post_parameters",
        serialize_with = "serialize_repeated_link_in_post_parameters",
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub links: std::collections::HashMap<u32, Option<String>>,
    #[doc = "Node id for this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodeid: Option<u64>,
    #[doc = "Number of votes for this node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub votes: Option<u64>,
}

pub fn deserialize_repeated_link_in_post_parameters<'de, D, V>(
    deserializer: D,
) -> Result<std::collections::HashMap<u32, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    V: serde::de::DeserializeOwned,
{
    crate::common::deserialize_repeated_with_prefix("link", deserializer)
}

fn serialize_repeated_link_in_post_parameters<V, S>(
    value: &std::collections::HashMap<u32, V>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: serde::Serialize,
    S: serde::Serializer,
{
    crate::common::serialize_repeated_with_prefix(value, "link", s)
}

impl PostParameters {
    pub fn new(clustername: String) -> Self {
        Self {
            clustername,
            links: Default::default(),
            nodeid: Default::default(),
            votes: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}

impl<T> ConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }

    pub fn apiversion(&self) -> apiversion::ApiversionClient<T> {
        apiversion::ApiversionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn nodes(&self) -> nodes::NodesClient<T> {
        nodes::NodesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn join(&self) -> join::JoinClient<T> {
        join::JoinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn totem(&self) -> totem::TotemClient<T> {
        totem::TotemClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn qdevice(&self) -> qdevice::QdeviceClient<T> {
        qdevice::QdeviceClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Generate new cluster configuration. If no links given, default to local IP address as link0."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncConfigClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncConfigClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "config"),
        }
    }

    pub fn apiversion(&self) -> apiversion::AsyncApiversionClient<T> {
        apiversion::AsyncApiversionClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn nodes(&self) -> nodes::AsyncNodesClient<T> {
        nodes::AsyncNodesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn join(&self) -> join::AsyncJoinClient<T> {
        join::AsyncJoinClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn totem(&self) -> totem::AsyncTotemClient<T> {
        totem::AsyncTotemClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn qdevice(&self) -> qdevice::AsyncQdeviceClient<T> {
        qdevice::AsyncQdeviceClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncConfigClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Generate new cluster configuration. If no links given, default to local IP address as link0."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
