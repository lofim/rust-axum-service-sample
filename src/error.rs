use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};

use http_api_problem::ApiError;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

pub type Result<T> = std::result::Result<T, Error>;
pub type HttpResult<T> = std::result::Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Requested resource '{name}' with ID: {id} not found")]
    ResourceNotFound { name: String, id: u32 },

    #[error("Error extracting json payload")]
    JSONExtractor(#[from] JsonRejection),

    #[error("Error extracting path parameters")]
    PathExtractor(#[from] PathRejection),

    #[error("Invalid request body")]
    Validator(#[from] ValidationErrors),

    #[error("Unexpected error")]
    Unexpected(anyhow::Error),
}

impl Error {
    pub fn error_code(&self) -> &str {
        match &self {
            Error::ResourceNotFound { name: _, id: _ } => "error.entity.not-found",
            Error::JSONExtractor(_) => "error.payload.invalid",
            Error::Validator(_) => "error.payload.invalid",
            Error::PathExtractor(_) => "error.path-parms.invalid",
            _ => "error.unexpected",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match &self {
            Error::ResourceNotFound { name: _, id: _ } => StatusCode::NOT_FOUND,
            Error::JSONExtractor(_) => StatusCode::BAD_REQUEST,
            Error::PathExtractor(_) => StatusCode::BAD_REQUEST,
            Error::Validator(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn details(&self) -> Option<String> {
        match &self {
            Error::JSONExtractor(error) => Some(error.body_text()),
            Error::PathExtractor(error) => Some(error.body_text()),
            Error::Validator(error) => Some(error.to_string()),
            _ => None,
        }
    }

    pub fn type_url(&self) -> String {
        format!("type://{}", &self.error_code())
    }

    pub fn from_with_context(error: sqlx::Error, entity_name: String, entity_id: u32) -> Error {
        match error {
            sqlx::Error::RowNotFound => Error::ResourceNotFound {
                name: entity_name,
                id: entity_id,
            },
            _ => Error::Unexpected(error.into()),
        }
    }
}

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        let mut api_error = ApiError::builder(value.status_code())
            .title(format!("{}", value))
            .type_url(value.type_url())
            .extension(value.error_code().to_owned());

        let details = value.details();
        if let Some(detail) = details {
            api_error = api_error.message(detail);
        }

        // Should probably move this into a global error handler
        // And convert the error::Error to ApiError after handlers are done
        if let Error::Unexpected(err) = value {
            error!("Encountered an error: {:?}", err);
        }

        api_error.finish()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let api_error: ApiError = self.into();
        api_error.into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Unexpected(value.into())
    }
}
