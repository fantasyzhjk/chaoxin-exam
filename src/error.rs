
use num_traits::ToPrimitive;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use std::io::Error as IoError;
use image::ImageError;
use rqrr::DeQRError;
use qrcode::types::QrError;
use playwright::Error as PlaywrightError;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub struct ApiError {
    pub code: u16,
    pub msg: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl ApiError {
    pub fn new<T: ToPrimitive + std::error::Error>(sub_err: T) -> Self {
        Self {
            code: sub_err.to_u16().unwrap(),
            msg: Some(sub_err.to_string()),
        }
    }

    pub fn custom(code: u16, msg: &str) -> Self {
        Self {
            code,
            msg: Some(msg.to_string()),
        }
    }
}

#[macro_export]
macro_rules! convert_inner_errors {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for ApiError {
            fn from(sub_err: $src_err_type) -> Self {
                Self {
                    code: 1,
                    msg: Some(sub_err.to_string()),
                }
            }
        }
    };
}
convert_inner_errors!(PlaywrightError);
convert_inner_errors!(ReqwestError);
convert_inner_errors!(ImageError);
convert_inner_errors!(DeQRError);
convert_inner_errors!(JsonError);
convert_inner_errors!(QrError);
convert_inner_errors!(IoError);

impl From<std::sync::Arc<playwright::Error>> for ApiError {
    fn from(sub_err: std::sync::Arc<playwright::Error>) -> Self {
        Self {
            code: 1,
            msg: Some(sub_err.to_string()),
        }
    }
}

// use pyo3::exceptions::PyException;
// use pyo3::create_exception;
// use pyo3::prelude::*;

// create_exception!(chaoxin_checkin, PyApiError, PyException);

// impl From<ApiError> for PyErr {
//     fn from(err: ApiError) -> PyErr {
//         PyApiError::new_err(err.to_string())
//     }
// }

// #[macro_export]
// macro_rules! py_try {
//     ($expr:expr $(,)?) => {
//         match $expr {
//             Ok(val) => val,
//             Err(err) => {
//                 return Err(PyErr::from(crate::error::ApiError::from(err)));
//             }
//         }
//     };
// }