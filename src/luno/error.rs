use serde_json::Error;
use url::ParseError;

#[derive(Debug)]
pub struct LunoErr {
    pub kind: ErrKind,
    pub message: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ErrKind {
    ErrInvalidArguments,
    ErrTravelRule,
    ErrInternal,
}
impl From<Error> for LunoErr {
    fn from(err: Error) -> Self {
        Self {
            kind: ErrKind::ErrInternal,
            message: err.to_string(),
        }
    }
}

impl From<String> for LunoErr {
    fn from(err: String) -> Self {
        Self {
            kind: ErrKind::ErrInvalidArguments,
            message: err,
        }
    }
}

impl From<ParseError> for LunoErr {
    fn from(err: ParseError) -> Self {
        Self {
            kind: ErrKind::ErrInternal,
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
