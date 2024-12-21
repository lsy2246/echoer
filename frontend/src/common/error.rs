use wasm_bindgen::JsValue;

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

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError(error.to_string())
    }
}

impl From<JsValue> for CustomError {
    fn from(error: JsValue) -> Self {
        CustomError(
            error
                .as_string()
                .unwrap_or_else(|| String::from("Unknown JS error")),
        )
    }
}

impl From<&str> for CustomError {
    fn from(error: &str) -> Self {
        CustomError(error.to_string())
    }
}

pub type CustomResult<T> = Result<T, CustomError>;
