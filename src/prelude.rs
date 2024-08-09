use crate::TuiError;

/// Application result type to keep errors consistent
pub type TuiResult<T> = std::result::Result<T, TuiError>;