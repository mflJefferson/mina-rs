use futures::future::err;
use serde::Serialize;
use std::error;
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

    pub fn status_code(&self) -> u16 {
        match self {
            Self::InvalidToken(_e) => 401,
            Self::ConnectionProblems(_error) => 404,
            Self::InvalidAddress(_error) => 401,
        }
    }

    pub fn to_json(&self) -> String {
        let message = match self {
            CustomResponseErrors::InvalidToken(error) => error.to_string(),
            CustomResponseErrors::ConnectionProblems(error) => error.to_string(),
            CustomResponseErrors::InvalidAddress(error) => error.to_string(),
        };

        let error_response = ErrorResponse {
            code: self.status_code(),
            message,
        };

        serde_json::to_string(&error_response).unwrap()
    }
}

impl Display for CustomResponseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_code())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}
