use std::fmt::{Display, Formatter};

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
    ErrQuoteVar,
    ExMacroDefrecord,
    ExMacroObjectCall
}

#[derive(Debug)]
pub enum ASTError {
    CSTError(CSTError),
    ErrSeq,
    ArgErrorDef,
    BadBindingForm,
    ErrLet(usize),
    ErrIf,
    ErrFn
}

impl Display for ASTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ASTError {}
