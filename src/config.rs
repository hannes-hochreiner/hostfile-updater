use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addresses: Vec<String>,
    pub hostnames: Vec<String>,
}
