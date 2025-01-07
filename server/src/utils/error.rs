use common::error::CommonError;
use rocket;
use surrealdb;

#[derive(Debug)]
pub struct CustomError(pub String);

pub trait CustomErrorInto {
    fn into_custom_error(self) -> CustomError;
}

impl CustomErrorInto for &str {
    fn into_custom_error(self) -> CustomError {
        CustomError(self.to_string())
    }
}

impl From<CommonError> for CustomError {
    fn from(error: CommonError) -> Self {
        CustomError(error.0)
    }
}

impl From<rocket::Error> for CustomError {
    fn from(error: rocket::Error) -> Self {
        CustomError(error.to_string())
    }
}

impl From<surrealdb::Error> for CustomError {
    fn from(error: surrealdb::Error) -> Self {
        CustomError(error.to_string())
    }
}

pub type CustomResult<T> = Result<T, CustomError>;
