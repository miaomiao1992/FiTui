use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tags: Vec<String>,
}

pub fn load_config() -> Config {
    let text = std::fs::read_to_string("config.yaml")
        .expect("Missing config.yaml");

    serde_yaml::from_str(&text)
        .expect("Invalid YAML format")
}
