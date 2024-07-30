pub mod test;

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }

    pub fn test(&self) -> test::TestClient<T> {
        test::TestClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NameClient<T> where T: crate::client::HttpClient {}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }

    pub fn test(&self) -> test::AsyncTestClient<T> {
        test::AsyncTestClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncNameClient<T> where T: crate::client::AsyncHttpClient {}
