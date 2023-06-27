use async_trait::async_trait;

use axum::{
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts},
    http::request::Parts,
    Json,
};

use hyper::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let value = axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        Ok(Self(value.0))
    }
}

pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for JsonExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = crate::error::Error;

    async fn from_request(
        req: Request<B>,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(JsonExtractor(value))
    }
}
