use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MainToml {
    name: String,
    main : String
}

impl MainToml {
    pub fn parse(string: &str) -> Result<MainToml, toml::de::Error> {
        toml::from_str(string) 
    }

    pub fn name(&self) -> &str {  &self.name  }

    pub fn main(&self) -> &str { &self.main  }
}
