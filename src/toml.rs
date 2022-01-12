use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct MainToml {
    package: String,
    main_entry : String
}

impl MainToml {
    pub fn parse(string: &str) -> MainToml {
        toml::from_str(string).unwrap()
    }
}
