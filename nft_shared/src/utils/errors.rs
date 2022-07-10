use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CustomResponseErrors {
    InvalidToken(web3::contract::Error),
    ConnectionProblems(String),
    InvalidAddress(String),
}

impl CustomResponseErrors {
    pub fn name(&self) -> String {
        match self {
            Self::InvalidToken(e) => e.to_string(),
            Self::ConnectionProblems(error) => error.to_string(),
            Self::InvalidAddress(error) => error.to_string(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomResponseErrors::ConnectionProblems(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomResponseErrors::InvalidAddress(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_json(&self) -> String {
        let message = match self {
            CustomResponseErrors::InvalidToken(error) => error.to_string(),
            CustomResponseErrors::ConnectionProblems(error) => error.to_string(),
            CustomResponseErrors::InvalidAddress(error) => error.to_string(),
        };

        let error_response = ErrorResponse {
            code: self.status_code().as_u16(),
            message,
        };

        serde_json::to_string(&error_response).unwrap()
    }
}

impl error::ResponseError for CustomResponseErrors {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomResponseErrors::ConnectionProblems(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomResponseErrors::InvalidAddress(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl Display for CustomResponseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_code())
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}
