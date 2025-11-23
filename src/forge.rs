use std::collections::HashMap;

use serde::{Deserialize, de};

use crate::forge::version::{ForgeVersion, ForgeVersionRange};

pub mod version;

pub type ModId = String;

/// A `META-INF/mods.toml` file.
/// Contains metadata about a forge mod.
#[derive(Debug)]
pub struct ForgeManifest {
    pub mod_loader: String,
    pub loader_version: ForgeVersionRange,
    pub license: String,
    pub issue_tracker_url: Option<String>,
    pub show_as_resource_pack: bool,
    pub properties: HashMap<String, String>,
    pub mods: Vec<Mod>,
    pub dependencies: HashMap<ModId, Vec<Dependency>>,
}

#[derive(Debug)]
pub struct Mod {
    pub mod_id: ModId,
    pub namespace: Option<String>,
    pub version: ForgeVersion,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub logo_file: Option<String>,
    pub logo_blur: bool,
    pub update_json_url: Option<String>,
    //pub modproperties: HashMap<>
    pub credits: Option<String>,
    pub authors: Option<String>,
    pub display_test: Option<String>,
}

/// `META-INF/mods.toml` before string substitution.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnsubstitutedForgeManifest {
    pub mod_loader: String,
    pub loader_version: ForgeVersionRange,
    pub license: String,
    #[serde(rename = "issueTrackerURL")]
    pub issue_tracker_url: Option<String>,
    #[serde(default)]
    pub show_as_resource_pack: bool,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    pub mods: Vec<UnsubstitutedMod>,
    pub dependencies: HashMap<ModId, Vec<Dependency>>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnsubstitutedMod {
    pub mod_id: ModId,
    pub namespace: Option<String>,
    #[serde(default = "default_version")]
    pub version: UnsubstitutedForgeVersion,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub logo_file: Option<String>,
    #[serde(default = "truthy")]
    pub logo_blur: bool,
    #[serde(rename = "updateJSONURL")]
    pub update_json_url: Option<String>,
    //pub modproperties: HashMap<>
    pub credits: Option<String>,
    pub authors: Option<String>,
    pub display_test: Option<String>,
}

const fn truthy() -> bool {
    true
}

fn default_version() -> UnsubstitutedForgeVersion {
    UnsubstitutedForgeVersion::ForgeVersion(ForgeVersion::parse("1").unwrap())
}

/// Can be a forge version or if the string "${file.jarVersion}" is used,
/// forge will replace the string with the 'Implementation Version' specified in the jar manifest.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnsubstitutedForgeVersion {
    ForgeVersion(ForgeVersion),
    ImplementationVersion,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub mod_id: ModId,
    pub mandatory: bool,
    pub version_range: ForgeVersionRange,
    #[serde(default)]
    pub ordering: Ordering,
    #[serde(default)]
    pub side: Side,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ordering {
    #[default]
    None,
    Before,
    After,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    #[default]
    Both,
    Client,
    Server,
}

impl<'de> Deserialize<'de> for UnsubstitutedForgeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct V;

        impl<'de> de::Visitor<'de> for V {
            type Value = UnsubstitutedForgeVersion;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a forge version")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "${file.jarVersion}" {
                    return Ok(UnsubstitutedForgeVersion::ImplementationVersion);
                }

                ForgeVersion::parse(v)
                    .map(UnsubstitutedForgeVersion::ForgeVersion)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl<'de> Deserialize<'de> for ForgeVersionRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl<'de> de::Visitor<'de> for V {
            type Value = ForgeVersionRange;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a forge version range")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                ForgeVersionRange::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_mods_toml() {
        let s = r#"
        modLoader="javafml"
        loaderVersion="[41,)"
        license="All rights reserved"
        issueTrackerURL="github.com/MinecraftForge/MinecraftForge/issues"

        [[mods]]
        modId="examplemod"
        version="1.0.0.0"

        [[mods]]
        modId="othermod"
        version="${file.jarVersion}"
        displayName="Other Mod"

        [[dependencies.examplemod]]
            modId="forge"
            mandatory=true
            versionRange="[41,)"

        [[dependencies.examplemod]]
            modId="minecraft"
            mandatory=true
            versionRange="[1.19,1.20)"
            ordering="BEFORE"
            side="SERVER""#;

        let v: UnsubstitutedForgeManifest = match toml::from_str(s) {
            Ok(value) => value,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(
            v,
            UnsubstitutedForgeManifest {
                mods: vec![
                    UnsubstitutedMod {
                        mod_id: "examplemod".into(),
                        version: UnsubstitutedForgeVersion::ForgeVersion(
                            ForgeVersion::parse("1.0.0.0").unwrap()
                        ),
                        display_name: None,
                        namespace: None,
                        description: None,
                        logo_file: None,
                        logo_blur: true,
                        update_json_url: None,
                        credits: None,
                        authors: None,
                        display_test: None
                    },
                    UnsubstitutedMod {
                        mod_id: "othermod".into(),
                        version: UnsubstitutedForgeVersion::ImplementationVersion,
                        namespace: None,
                        display_name: Some("Other Mod".into()),
                        description: None,
                        logo_file: None,
                        logo_blur: true,
                        update_json_url: None,
                        credits: None,
                        authors: None,
                        display_test: None
                    }
                ],
                dependencies: HashMap::from([(
                    "examplemod".into(),
                    vec![
                        Dependency {
                            mod_id: "forge".into(),
                            mandatory: true,
                            version_range: ForgeVersionRange::parse("[41,)").unwrap(),
                            ordering: Ordering::None,
                            side: Side::Both,
                        },
                        Dependency {
                            mod_id: "minecraft".into(),
                            mandatory: true,
                            version_range: ForgeVersionRange::parse("[1.19,1.20)").unwrap(),
                            ordering: Ordering::Before,
                            side: Side::Server,
                        }
                    ]
                ),]),
                mod_loader: "javafml".into(),
                loader_version: ForgeVersionRange::parse("[41,)").unwrap(),
                license: "All rights reserved".into(),
                issue_tracker_url: Some("github.com/MinecraftForge/MinecraftForge/issues".into()),
                show_as_resource_pack: false,
                properties: HashMap::new(),
            }
        )
    }
}
