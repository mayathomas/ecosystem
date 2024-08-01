use anyhow::Result;
use strum::{
    EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr,
    VariantNames,
};

#[allow(unused)]
#[derive(
    Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames,
)]
enum MyEnum {
    A,
    B(String),
    C,
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{:?}", v));

    let b = MyEnum::B("hello".to_string());
    println!("{:?}", b.is_b());

    let bstr: &'static str = b.into();
    println!("{}", bstr);

    Ok(())
}
