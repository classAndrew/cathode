use std::{error::Error, any::type_name};

use axum::{response::{IntoResponse, Response}, http::StatusCode};
use tracing::log::warn;

#[derive(Debug)]
pub enum CathodeError {
    InternalError(String),
    RequestError(String)
}

impl From<CathodeError> for Response {
    fn from(error: CathodeError) -> Self {
        error.into_response()
    }
}

impl<E> From<E> for CathodeError 
where
    E: Error
{
    fn from(error: E) -> Self {
        let msg = format!("[{:?}] error: {:?}", type_name::<E>(), error.to_string());
        warn!("{}", msg);
        CathodeError::InternalError(msg)
    }
}

impl IntoResponse for CathodeError {
    fn into_response(self) -> Response {
        let (response_code, body) = match self {
            CathodeError::InternalError(msg) => 
                (StatusCode::INTERNAL_SERVER_ERROR, msg),
            CathodeError::RequestError(msg) => 
                (StatusCode::BAD_REQUEST, msg),
        };

        (response_code, body).into_response()
    }
}