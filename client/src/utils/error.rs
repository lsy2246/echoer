use wasm_bindgen::JsValue;
use common::error::CommonError;

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

impl From<JsValue> for CustomError {
    fn from(error: JsValue) -> Self {
        CustomError(
            error
                .as_string()
                .unwrap_or_else(|| String::from("Unknown JS error")),
        )
    }
}

pub type CustomResult<T> = Result<T, CustomError>;

