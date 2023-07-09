/**
* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Project Structure
///
/// ```yaml
/// name: "A Verde Project"
///
/// tree:
///  ReplicatedStorage:
///    .path: src/shared
///
///  ServerScriptService:
///    .path: src/server
/// ````
#[derive(Serialize, Deserialize)]
pub struct VerdeProject {
    /// Name of project
    pub name: String,

    /// The instance tree
    pub tree: BTreeMap<String, Node>,
}

/// An instance node.
#[derive(Serialize, Deserialize)]
pub struct Node {
    /// Path (relative to source directory)
    #[serde(rename = ".path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    // Properties applied to the related Roblox instance
    #[serde(rename = ".properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,

    /// Additonal instance tree
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub contents: Option<BTreeMap<String, Node>>,
}

impl Default for VerdeProject {
    fn default() -> Self {
        Self {
            name: String::from("A Verde Project"),
            tree: BTreeMap::<String, Node>::from([
                (
                    String::from("ServerScriptService"),
                    Node {
                        path: Some(String::from("src/server")),
                        properties: None,
                        contents: None,
                    },
                ),
                (
                    String::from("ReplicatedStorage"),
                    Node {
                        path: Some(String::from("src/shared")),
                        properties: None,
                        contents: None,
                    },
                ),
                (
                    String::from("StarterPlayer"),
                    Node {
                        path: None,
                        properties: None,
                        contents: Some(BTreeMap::<String, Node>::from([(
                            String::from("StarterPlayerScripts"),
                            Node {
                                path: Some(String::from("src/client")),
                                properties: None,
                                contents: None,
                            },
                        )])),
                    },
                ),
            ]),
        }
    }
}