use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "private_build"), pattern = "owned")]
struct User {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into, strip_option), default)]
    email: Option<String>,

    #[builder(setter(custom))]
    dob: DateTime<Utc>,

    #[builder(setter(skip))]
    age: u32,

    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(self) -> Result<User> {
        let mut user = self.private_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }
    pub fn dob(self, value: &str) -> Self {
        let dob = DateTime::parse_from_rfc3339(value)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();
        Self { dob, ..self }
    }
}

fn main() -> Result<()> {
    let user = User::build()
        .name("maya")
        .skill("Rust")
        .skill("C++")
        .email("maya@qq.com")
        .dob("1996-08-21T00:00:00+08:00")
        .build()?;
    println!("{:?}", user);
    Ok(())
}
