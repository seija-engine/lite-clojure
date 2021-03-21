#[derive(Debug)]
pub enum CSTError {
    InvalidChar(char),
    ErrExpectedHex,
    ErrLeadingZero,
    ErrExpectedExponent,
    ErrNumberOutOfRange,
    ErrLineFeedInString,
    ErrCharInGap(char),
    ErrLexeme(Option<String>),
    UnsupportedCharacter(String),
    ErrToken(String),
    ErrMetadata,
    ErrEof
}

#[derive(Debug)]
pub enum ASTError {

}