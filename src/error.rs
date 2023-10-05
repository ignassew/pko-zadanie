use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum DataError {
    UserNotFound,
}

pub type DataResult<T> = Result<T, DataError>;

impl IntoResponse for DataError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DataError::UserNotFound => {
                (StatusCode::NOT_FOUND, "User not found")
            }
        };

        (status, error_message).into_response()
    }
}
