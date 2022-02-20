use figment::{providers::Env, Figment};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn figment() -> Figment {
        Figment::new().merge(Env::prefixed("APP_").global())
    }
}
