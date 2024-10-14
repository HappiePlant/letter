use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) repo: Vec<Repo>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Repo {
    pub id: String,

    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    pub url: Url,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            repo: Vec::new(),
        }
    }
}

fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&url.to_string())
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    Url::parse(&string).map_err(serde::de::Error::custom)
}