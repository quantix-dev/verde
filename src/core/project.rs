/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::core::node::base::Node;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
  collections::BTreeMap,
  fs::{self, File},
  io::Read,
  path::Path,
  result::Result::Ok,
};

pub const DEFAULT_PROJECT: &str = "verde.yaml";

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
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerdeProject {
  /// Name of project
  pub name: String,

  /// The instance tree
  pub tree: Node,
}

// Verde project implementation
impl VerdeProject {
  /// Creates a VerdeProject using defaults.
  pub fn new() -> Self {
    Self {
      name: String::from("A Verde Project"),
      tree: Node {
        class_name: Some(String::from("DataModel")),
        path: None,
        overwrite_descendants: None,
        contents: Some(BTreeMap::<String, Node>::from([
          (
            String::from("ServerScriptService"),
            Node {
              class_name: Some(String::from("ServerScriptService")),
              path: Some(String::from("src/server")),
              overwrite_descendants: None,
              contents: None,
            },
          ),
          (
            String::from("ReplicatedStorage"),
            Node {
              class_name: Some(String::from("ReplicatedStorage")),
              path: Some(String::from("src/shared")),
              overwrite_descendants: None,
              contents: Some(BTreeMap::<String, Node>::from([(
                String::from("client"),
                Node {
                  class_name: None,
                  path: Some(String::from("src/client")),
                  overwrite_descendants: None,
                  contents: None,
                },
              )])),
            },
          ),
        ])),
      },
    }
  }

  /// Creates a new VerdeProject from the specified file.
  pub fn from(project: &mut File) -> anyhow::Result<Self> {
    let mut buffer = String::new();
    project
      .read_to_string(&mut buffer)
      .with_context(|| format!("Failed to read file {:#?}", project))?;

    serde_yaml::from_str(&buffer).context("Failed to deserialise yaml to VerdeProject.")
  }

  /// Saves the VerdeProject to the file system.
  pub fn save(&self) -> anyhow::Result<()> {
    let dest = Path::new(DEFAULT_PROJECT);
    self.save_to(dest)?;
    Ok(())
  }

  /// Saves the VerdeProject to the specified file location.
  pub fn save_to(&self, destination: &Path) -> anyhow::Result<()> {
    let content = serde_yaml::to_string(self)?;
    fs::write(destination, content).with_context(|| format!("Failed to write project to {}", destination.display()))?;

    Ok(())
  }
}

impl Default for VerdeProject {
  fn default() -> Self {
    Self::new()
  }
}
