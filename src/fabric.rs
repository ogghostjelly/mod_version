use std::{collections::HashMap, path::PathBuf};

use serde::{
    Deserialize,
    de::{self, Error as _},
};

pub mod version;

use version::FabricVersion;

use crate::fabric::version::FabricVersionRange;

/// A `fabric.mod.json` file.
/// Contains metadata about a fabric mod.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModJson {
    pub schema_version: SchemaVersion,
    pub id: ModId,
    pub version: FabricVersion,
    pub provides: Option<Vec<ModId>>,
    pub environment: Option<Environment>,
    pub jars: Option<Vec<JarPath>>,

    pub entrypoints: (),
    pub language_adapters: (),
    pub mixins: (),

    pub depends: HashMap<ModId, FabricVersionRange>,
    pub recommends: HashMap<ModId, FabricVersionRange>,
    pub suggests: HashMap<ModId, FabricVersionRange>,
    pub breaks: HashMap<ModId, FabricVersionRange>,
    pub conflicts: HashMap<ModId, FabricVersionRange>,

    pub name: (),
    pub description: (),
    pub contact: (),
    pub authors: (),
    pub contributors: (),
    pub license: (),
    pub icon: (),
}

#[derive(Deserialize)]
pub struct JarPath {
    pub file: PathBuf,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Client,
    Server,
    #[serde(alias = "*")]
    Any,
}

#[derive(Deserialize)]
#[serde(try_from = "u64")]
pub struct SchemaVersion;

impl TryFrom<u64> for SchemaVersion {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 1 {
            Ok(Self)
        } else {
            Err(Error::InvalidSchemaVersion)
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "&str")]
pub struct ModId(pub String);

impl TryFrom<&str> for ModId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if (2..=64).contains(&value.len()) {
            Ok(ModId(value.to_string()))
        } else {
            Err(Error::ModIdTooLong)
        }
    }
}

impl<'de> Deserialize<'de> for FabricVersionRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct V;

        impl<'de> de::Visitor<'de> for V {
            type Value = FabricVersionRange;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a version string or list of versions")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FabricVersionRange::parse_single(v).map_err(E::custom)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut ls: Vec<&str> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(value) = seq.next_element()? {
                    ls.push(value);
                }
                FabricVersionRange::parse_many(ls.into_iter()).map_err(A::Error::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl<'de> Deserialize<'de> for FabricVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl<'de> de::Visitor<'de> for V {
            type Value = FabricVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a fabric version string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FabricVersion::parse(v, false).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("mod id must be between 2-64 characters")]
    ModIdTooLong,
    #[error("invalid schema version is not 1")]
    InvalidSchemaVersion,
}
