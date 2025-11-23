use crate::forge::{self, version::ForgeVersion};

pub fn parse(s: &str) -> impl Iterator<Item = Result<(&str, &str), Error>> {
    let iter = s.lines();

    iter.map(|line| match line.split_once(':') {
        Some((k, v)) => Ok((k.trim(), v.trim())),
        None => Err(Error::MissingDelimeter),
    })
}

pub fn extract_implementation_version(s: &str) -> Result<ForgeVersion, Error> {
    for line in parse(s) {
        let (key, value) = line?;
        if key == "Implementation-Version" {
            return Ok(ForgeVersion::parse(value)?);
        }
    }
    Err(Error::MissingImplementationVersion)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing ':' property delimeter")]
    MissingDelimeter,
    #[error("META-INF/MANIFEST.MF missing Implementation-Version")]
    MissingImplementationVersion,
    #[error("Implementation-Version: {0}")]
    Forge(#[from] forge::version::Error),
}
