#[derive(Debug)]
pub enum Error {
    StackOverflow,
    Message(String)
}