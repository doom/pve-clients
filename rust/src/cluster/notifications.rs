pub mod endpoints;
pub mod matchers;
pub mod targets;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct NotificationsClient<T> {
    client: T,
    path: String,
}

impl<T> NotificationsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "notifications"),
        }
    }

    pub fn endpoints(&self) -> endpoints::EndpointsClient<T> {
        endpoints::EndpointsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn targets(&self) -> targets::TargetsClient<T> {
        targets::TargetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn matchers(&self) -> matchers::MatchersClient<T> {
        matchers::MatchersClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NotificationsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index for notification-related API endpoints."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNotificationsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNotificationsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "notifications"),
        }
    }

    pub fn endpoints(&self) -> endpoints::AsyncEndpointsClient<T> {
        endpoints::AsyncEndpointsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn targets(&self) -> targets::AsyncTargetsClient<T> {
        targets::AsyncTargetsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn matchers(&self) -> matchers::AsyncMatchersClient<T> {
        matchers::AsyncMatchersClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncNotificationsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index for notification-related API endpoints."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
