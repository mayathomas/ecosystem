use anyhow::Result;
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Clone, Copy, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq, From)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() -> Result<()> {
    let my_int: MyInt = 10.into();
    let v = my_int + 20.into();

    println!("my_int: {}, v: {}", my_int, v);
    Ok(())
}
