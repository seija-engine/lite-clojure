#[derive(Debug)]
pub enum EvalError {
    ZeroFnList,
    NotFoundSymbol(String),
    ListFirstMustFunction,
    FunctionArgCountError
}