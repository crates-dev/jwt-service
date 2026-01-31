use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum JwtValidationError {
    Expired,
    InvalidSignature,
    InvalidIssuer,
    InvalidSubject,
    NotYetValid,
    Malformed,
    Other(String),
}
