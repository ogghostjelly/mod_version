use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManifestJson {
    pub minecraft: Minecraft,
    pub manifest_type: String,
    pub manifest_version: u64,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<File>,
    pub overrides: PathBuf,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct File {
    #[serde(rename = "projectID")]
    pub project_id: u64,
    #[serde(rename = "fileID")]
    pub file_id: u64,
    pub required: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Minecraft {
    pub version: String,
    pub mod_loaders: Vec<ModLoader>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ModLoader {
    pub id: String,
    pub primary: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_manifest_json() {
        let s = r#"{
            "minecraft": {
                "version": "1.20.1",
                "modLoaders": [
                {
                    "id": "fabric-0.14.23",
                    "primary": true
                }
                ]
            },
            "manifestType": "minecraftModpack",
            "manifestVersion": 1,
            "name": "Fabulously Optimized",
            "version": "5.4.1",
            "author": "robotkoer",
            "files": [
                {
                "projectID": 374274,
                "fileID": 4581910,
                "required": true
                }
            ],
            "overrides": "overrides"
        }"#;
        assert_eq!(
            serde_json::from_str::<ManifestJson>(s).unwrap(),
            ManifestJson {
                minecraft: Minecraft {
                    version: "1.20.1".into(),
                    mod_loaders: vec![ModLoader {
                        id: "fabric-0.14.23".into(),
                        primary: true
                    }],
                },
                manifest_type: "minecraftModpack".into(),
                manifest_version: 1,
                name: "Fabulously Optimized".into(),
                version: "5.4.1".into(),
                author: "robotkoer".into(),
                files: vec![File {
                    project_id: 374274,
                    file_id: 4581910,
                    required: true
                }],
                overrides: "overrides".into(),
            }
        )
    }
}
