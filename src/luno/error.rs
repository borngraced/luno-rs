use serde_json::Error;
use url::ParseError;

#[derive(Debug)]
pub struct LunoErr {
    pub kind: ErrKind,
    pub message: String,
}


#[derive(Debug)]
pub enum ErrKind {
    InvalidArguments,
    TravelRule,
    Internal,
}
impl From<Error> for LunoErr {
    fn from(err: Error) -> Self {
        Self {
            kind: ErrKind::Internal,
            message: err.to_string(),
        }
    }
}

impl From<String> for LunoErr {
    fn from(err: String) -> Self {
        Self {
            kind: ErrKind::InvalidArguments,
            message: err,
        }
    }
}

impl From<ParseError> for LunoErr {
    fn from(err: ParseError) -> Self {
        Self {
            kind: ErrKind::Internal,
            message: err.to_string(),
        }
    }
}

#[allow(unused)]
impl LunoErr {
    fn new(kind: ErrKind, message: String) -> Self {
        Self { kind, message }
    }

    fn display_err(&self) -> String {
        format!("Kind: {:?} \n Message: {}", self.kind, self.message)
    }
}
