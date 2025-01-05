#[derive(Debug)]
pub struct CommonError(pub String);

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for CommonError {
    fn from(error: std::io::Error) -> Self {
        CommonError(error.to_string())
    }
}

impl From<String> for CommonError {
    fn from(error: String) -> Self {
        CommonError(error)
    }
}

impl From<&str> for CommonError {
    fn from(error: &str) -> Self {
        CommonError(error.to_string())
    }
}
