use lite_clojure_eval::EvalError;
use thiserror::{Error};
#[derive(Error,Debug)]
pub enum FRPError {
    #[error("not found system")]
    NotFoundSystem,
    #[error("type cast error")]
    TypeCastError,
    #[error("event not found")]
    EventNotFound,
    #[error("dynamic not found")]
    DynamicNotFound,
    #[error("eval error:{0:?}")]
    EvalError(EvalError)
}