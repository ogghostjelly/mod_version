use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

/// Modrinth modpack manifest file: `modrinth.index.json`
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MrManifest {
    pub format_version: u64,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<File>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub path: PathBuf,
    pub hashes: HashMap<String, String>,
    pub env: Option<Env>,
    pub downloads: Vec<String>,
    pub file_size: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Env {
    pub client: ProjectSupportRange,
    pub server: ProjectSupportRange,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSupportRange {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_modrinth_index_json() {
        let s = r#"{
            "formatVersion": 1,
            "game": "minecraft",
            "versionId": "5.4.1",
            "name": "Fabulously Optimized",
            "summary": "Improve your graphics and performance with this simple modpack.",
            "files": [
                {
                    "path": "mods/advancementinfo-1.20-fabric0.83.0-1.4.jar",
                    "hashes": {
                        "sha1": "dfa603a2db09d6e303dd2991f016550ae156e3d1",
                        "sha512": "ff10a77f831354f757c1ba4b6906e612549f3ac0dd3b15101dae29a459937bfceabe5e7ce7cd9126e58833a69e5a4ba04993dfcf2098830b9ccf2c192dddc0e5"
                    },
                    "downloads": [
                        "https://cdn.modrinth.com/data/G1epq3jN/versions/gfcbMV82/advancementinfo-1.20-fabric0.83.0-1.4.jar"
                    ],
                    "fileSize": 43895
                }
            ],
            "dependencies": {
                "minecraft": "1.20.1",
                "fabric-loader": "0.14.23"
            }
        }"#;
        let MrManifest {
            format_version,
            game,
            version_id,
            name,
            summary,
            files,
            dependencies,
        } = serde_json::from_str::<MrManifest>(s).unwrap();

        assert_eq!(format_version, 1);
        assert_eq!(game, "minecraft");
        assert_eq!(version_id, "5.4.1");
        assert_eq!(name, "Fabulously Optimized");
        assert_eq!(
            summary,
            Some("Improve your graphics and performance with this simple modpack.".into())
        );
        assert_eq!(files, vec![File {
                    path: "mods/advancementinfo-1.20-fabric0.83.0-1.4.jar".into(),
                    hashes: HashMap::from([
                        ("sha1".into(), "dfa603a2db09d6e303dd2991f016550ae156e3d1".into()),
                        ("sha512".into(), "ff10a77f831354f757c1ba4b6906e612549f3ac0dd3b15101dae29a459937bfceabe5e7ce7cd9126e58833a69e5a4ba04993dfcf2098830b9ccf2c192dddc0e5".into())
                    ]),
                    env: None,
                    downloads: vec!["https://cdn.modrinth.com/data/G1epq3jN/versions/gfcbMV82/advancementinfo-1.20-fabric0.83.0-1.4.jar".into()],
                    file_size: 43895
                }]);
        assert_eq!(
            dependencies,
            HashMap::from_iter([
                ("minecraft".into(), "1.20.1".into()),
                ("fabric-loader".into(), "0.14.23".into()),
            ])
        );
    }
}
