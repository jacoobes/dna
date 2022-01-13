use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MainToml {
    pub package: String,
    main : String
}

impl MainToml {
    pub fn parse(string: &str) -> Result<MainToml, toml::de::Error> {
        toml::from_str(string) 
    }
}
