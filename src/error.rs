use failure::Fail;
use jsonrpc_http_server::jsonrpc_core::{Error as RpcError, ErrorCode};

#[derive(Debug, Fail, Eq, PartialEq)]
pub enum Error {
    #[fail(
        display = "Request lock_hash '{}' must be 32bytes hex string starting with 0x",
        _0
    )]
    RequestParamHexInvalid(String),

    #[fail(display = "The lock_hash has registered")]
    LockHashHasRegistered,

    #[fail(display = "Database '{}' query error", _0)]
    DatabaseQueryError(String),
}

impl Error {
    pub fn to_msg(self) -> String {
        match self {
            Self::RequestParamHexInvalid(msg) => format!(
                "Request lock_hash '{}' must be 32bytes hex string starting with 0x",
                msg
            ),
            Self::LockHashHasRegistered => "The lock_hash has registered".to_string(),
            Self::DatabaseQueryError(_) => "Internal error".to_string(),
        }
    }
}

impl Into<RpcError> for Error {
    fn into(self) -> RpcError {
        RpcError {
            code:    ErrorCode::InvalidParams,
            message: self.to_msg(),
            data:    None,
        }
    }
}
