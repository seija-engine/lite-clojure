#[derive(Debug)]
pub enum CSTError {
    InvalidSymbolChar(char),
    InvalidChar(char),
    ErrExpectedHex,
    ErrLeadingZero,
    ErrExpectedExponent,
    ErrNumberOutOfRange,
    ErrLineFeedInString,
    ErrCharInGap(char),
    ErrLexeme(Option<String>),
    UnsupportedCharacter(String),
    ErrSymbol(String),
    ErrMetadata,
    ErrEof,
    ErrQuoteVar
}

#[derive(Debug)]
pub enum ASTError {
    CSTError(CSTError),
    ErrSeq,
    ArgErrorDef,
    BadBindingForm,
    ErrLet,
    ErrIf,
}