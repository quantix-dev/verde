/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
pub mod node;
pub mod project;
pub mod session;
pub mod sourcemap;

use crate::core::project::VerdeProject;
use crate::core::session::{SessionState, VerdeSession};
use anyhow::bail;
use std::fs::File;
use std::sync::Arc;

pub struct VerdeCore {
  /// Current loaded project file
  pub project: Option<Arc<VerdeProject>>,

  pub session: Option<VerdeSession>,
}

// TODO: I think this is redundent and session/project should be their own thing
//       Session relies on project so making them in this structure makes it difficult to pass them around
impl VerdeCore {
  pub fn new() -> Self {
    VerdeCore {
      project: None,
      session: None,
    }
  }

  /// Sets the current Verde Project
  pub fn project(&mut self, path: &str) -> anyhow::Result<&mut Self> {
    match self.project {
      Some(_) => println!("A project has already been specified"),
      None => {
        let mut project_file = File::open(path)?;
        let project = VerdeProject::from(&mut project_file)?;
        self.project = Some(Arc::new(project));
      }
    }

    Ok(self)
  }

  /// Starts a new Verde Session
  pub fn start_session(&self) -> anyhow::Result<&Self> {
    if let Some(project) = &self.project {
      let session = VerdeSession::new(project);
      match session.state {
        SessionState::Active => println!("Session is already active"),
        SessionState::Offline => session.start(),
        SessionState::Error => println!("Session has entered an errored state."),
      };
    } else {
      bail!("Please ensure a project is loaded before starting a session.");
    }

    Ok(self)
  }
}

impl Default for VerdeCore {
  fn default() -> Self {
    Self::new()
  }
}
