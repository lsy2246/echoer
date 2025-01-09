use common::error::CommonError;
use dioxus::prelude::RenderError;
use wasm_bindgen::JsValue;

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

impl From<JsValue> for CustomError {
    fn from(error: JsValue) -> Self {
        CustomError(
            error
                .as_string()
                .unwrap_or_else(|| String::from("Unknown JS error")),
        )
    }
}

impl From<RenderError> for CustomError {
    fn from(error: RenderError) -> Self {
        CustomError(error.to_string())
    }
}

pub type CustomResult<T> = Result<T, CustomError>;
