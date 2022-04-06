#[derive(Debug)]
pub enum EvalError {
    ZeroFnList,
    NotFoundSymbol(String),
    TypeCastError,
    ListFirstMustFunction,
    FunctionArgCountError
}