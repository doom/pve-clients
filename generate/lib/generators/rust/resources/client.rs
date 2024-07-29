use serde::{de::DeserializeOwned, Serialize};

pub trait HttpClient {
    type Error;

    fn get<Q, R>(&self, path: &str, query: Q) -> Result<R, Self::Error>
    where
        Q: Serialize,
        R: DeserializeOwned;

    fn post<B, R>(&self, path: &str, body: B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned;

    fn put<B, R>(&self, path: &str, body: B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned;

    fn delete<B, R>(&self, path: &str, body: B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned;
}

pub trait AsyncHttpClient {
    type Error;

    fn get<Q, R>(
        &self,
        path: &str,
        query: Q,
    ) -> impl std::future::Future<Output = Result<R, Self::Error>> + Send
    where
        Q: Serialize + Send,
        R: DeserializeOwned + Send;

    fn post<B, R>(
        &self,
        path: &str,
        body: B,
    ) -> impl std::future::Future<Output = Result<R, Self::Error>> + Send
    where
        B: Serialize + Send,
        R: DeserializeOwned + Send;

    fn put<B, R>(
        &self,
        path: &str,
        body: B,
    ) -> impl std::future::Future<Output = Result<R, Self::Error>> + Send
    where
        B: Serialize + Send,
        R: DeserializeOwned + Send;

    fn delete<B, R>(
        &self,
        path: &str,
        body: B,
    ) -> impl std::future::Future<Output = Result<R, Self::Error>> + Send
    where
        B: Serialize + Send,
        R: DeserializeOwned + Send;
}
