/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper module for git written in Rust                              ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use std::os::linux::raw::stat;
use std::path::{PathBuf};
use std::process::{Command};
use crate::kot::err;
use super::kerror::{Error, ErrorKind};

use super::kfs;


// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

/// Clones a Git repository using https or ssh
/// + By default, cloned repositories are stored in $HOME/.local/share/kot/dotfiles/
pub fn clone(repo_url: &str, clone_dir: &PathBuf)
  -> super::Result<PathBuf> {
  // Clone the repository, check that status return value is 0
  let status = Command::new("git")
      .args(["clone", repo_url, clone_dir.to_str().unwrap(), "--recursive"])
      .status().unwrap();

  return match status.code() {
    Some(0) => Ok(clone_dir.to_owned()),
    _ => {
      return
          err!(ErrorKind::GitError(status.to_string()),
                   format!("Unable to clone repository"));
    }
  }
}

/// Extracts repository name from URL
pub fn repo_name(repo_url: &str) -> String {
  return repo_url.rsplit_once('/').unwrap().1
      .strip_suffix(".git").unwrap().to_owned();
}
