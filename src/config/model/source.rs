use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Clone)]
pub enum SourcePath {
    Local(PathBuf),
    Remote(Url),
}

impl FromStr for SourcePath {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = Url::parse(s) {
            if matches!(url.scheme(), "http" | "https") {
                return Ok(SourcePath::Remote(url));
            }
        }
        Ok(SourcePath::Local(PathBuf::from(s)))
    }
}

impl<'de> Deserialize<'de> for SourcePath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(s.parse().unwrap())
    }
}

impl fmt::Display for SourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourcePath::Local(p) => write!(f, "{}", p.display()),
            SourcePath::Remote(u) => write!(f, "{u}"),
        }
    }
}
