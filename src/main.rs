use anyhow::{Context, Result};
use ecosystem::MyError;
use std::fs;

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
