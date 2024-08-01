use anyhow::{Context, Result};
use std::fs;

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

fn main() -> Result<(), anyhow::Error> {
    println!("MyError size: {}", std::mem::size_of::<MyError>());

    let filename = "non-existent.txt";
    // let fd = fs::File::open(filename).context(format!("Can not find file: {}", filename));
    let _fd = fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename));

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("Failed with error".to_string()))
}
