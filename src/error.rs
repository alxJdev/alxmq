use std::backtrace::Backtrace;

#[derive(Debug)]
pub struct InternalError {
    pub msg: String,
    pub backtrace: Backtrace,
}

impl InternalError {
    pub fn new_basic_error() -> InternalError {
        InternalError {
            msg: "A Basic Error Occurred".to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}