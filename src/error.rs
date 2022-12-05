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

    #[fail(display = "'{}' SMT error", _0)]
    SMTError(String),

    #[fail(display = "CKB Indexer error: {}", _0)]
    CKBIndexerError(String),

    #[fail(display = "'{}' RocksDB error", _0)]
    RocksDBError(String),
}

impl Error {
    pub fn to_msg(self) -> String {
        match self {
            Self::RequestParamHexInvalid(msg) => format!(
                "Request lock_hash '{}' must be 32bytes hex string starting with 0x",
                msg
            ),
            Self::LockHashHasRegistered => {
                "The lock_hash has registered".to_string()
            }
            Self::CKBIndexerError(msg) => format!("CKB Indexer error: {}", msg),
            Self::DatabaseQueryError(msg) => format!("Database query error: {}", msg),
            Self::SMTError(msg) => format!("SMT error: {}", msg),
            Self::RocksDBError(msg) => format!("RocksDB error: {}", msg),
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
