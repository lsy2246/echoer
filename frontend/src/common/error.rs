#[derive(Debug)]
pub struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait CustomErrorInto {
    fn into_custom_error(self) -> CustomError;
}

impl CustomErrorInto for &str {
    fn into_custom_error(self) -> CustomError {
        CustomError(self.to_string())
    }
}

impl<E: std::error::Error> From<E> for CustomError {
    fn from(error: E) -> Self {
        CustomError(error.to_string())
    }
}

pub type CustomResult<T> = Result<T, CustomError>;

