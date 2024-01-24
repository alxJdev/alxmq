#[derive(Debug)]
pub struct InternalError {
    pub msg: String,
}

impl InternalError {
    pub fn new_basic_error() -> InternalError {
        InternalError {
            msg: "A Basic Error Occurred".to_string(),
        }
    }
}