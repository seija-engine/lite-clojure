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
    ErrEof
}

#[derive(Debug)]
pub enum ASTError {

}