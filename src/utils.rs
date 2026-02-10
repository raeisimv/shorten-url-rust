use axum::http::StatusCode;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub type MyError = Box<dyn std::error::Error>;
pub type MyResult<T = (), E = MyError> = Result<T, E>;
