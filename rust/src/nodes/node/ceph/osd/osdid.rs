pub mod r#in;
pub mod lv_info;
pub mod metadata;
pub mod out;
pub mod scrub;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteParameters {
    #[doc = "If set, we remove partition table entries."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub cleanup: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct OsdidClient<T> {
    client: T,
    path: String,
}

impl<T> OsdidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, osdid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, osdid),
        }
    }

    pub fn metadata(&self) -> metadata::MetadataClient<T> {
        metadata::MetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lv_info(&self) -> lv_info::LvInfoClient<T> {
        lv_info::LvInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn r#in(&self) -> r#in::InClient<T> {
        r#in::InClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn out(&self) -> out::OutClient<T> {
        out::OutClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn scrub(&self) -> scrub::ScrubClient<T> {
        scrub::ScrubClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy OSD"]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "OSD index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOsdidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOsdidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, osdid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, osdid),
        }
    }

    pub fn metadata(&self) -> metadata::AsyncMetadataClient<T> {
        metadata::AsyncMetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn lv_info(&self) -> lv_info::AsyncLvInfoClient<T> {
        lv_info::AsyncLvInfoClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn r#in(&self) -> r#in::AsyncInClient<T> {
        r#in::AsyncInClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn out(&self) -> out::AsyncOutClient<T> {
        out::AsyncOutClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn scrub(&self) -> scrub::AsyncScrubClient<T> {
        scrub::AsyncScrubClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncOsdidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy OSD"]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "OSD index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
