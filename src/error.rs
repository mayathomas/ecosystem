use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Serializer json error: {0}")]
    Serializer(#[from] serde_json::Error),

    #[error("Big error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: u64,
}

// impl From <std::io::Error> for MyError {
//     fn from(s: std::io::Error) -> Self {
//         MyError::Custom(s.to_string())
//     }
// }
