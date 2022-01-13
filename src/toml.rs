use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MainToml {
    package: String,
    main : String
}

impl MainToml {
    pub fn parse(string: &str) -> Result<MainToml, toml::de::Error> {
        toml::from_str(string) 
    }

    pub fn package(&self) -> &str {  &self.package  }

    pub fn main(&self) -> &str { &self.main  }
}
