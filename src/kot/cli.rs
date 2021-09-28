/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Wrapper for StructOpt crate functionality used by kot               ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use structopt::StructOpt;

// =============================================================================
// STRUCTS
// =============================================================================

// -----------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(
    name="kot",
    about="CLI for managing Linux user configurations"
)]
pub struct Cli {
    #[structopt(
        help="Local or full path to user configurations to install",
        parse(from_os_str)
    )]
    pub configs_dir: std::path::PathBuf,

    #[structopt(
        help="The location to attempt installation of user configurations",
        default_value="dry-runs/kapper", // TODO: Remove temp default value after tests
        // env = "HOME", // Default value to env variable $HOME
        long="home-dir",
        parse(from_os_str)
    )]
    pub install_dir: std::path::PathBuf,

    #[structopt(
        help="The location to store backups for this user",
        default_value="backups/kapper",
        long="backup-dir",
        parse(from_os_str)
    )]
    pub backup_dir: std::path::PathBuf,
}

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------

// Augment implementation of from_args to limit scope of StructOpt
// + Also enforces use of Cli::normalize()
// https://docs.rs/structopt/0.3.23/src/structopt/lib.rs.html#1121-1126
pub fn from_args() -> Cli {
    let s = Cli::from_clap(&Cli::clap().get_matches());
    s.normalize()
}

impl Cli {
    // Helper function to normalize arguments passed to program
    pub fn normalize(mut self) -> Self {
        self.configs_dir = self.configs_dir.canonicalize().unwrap();
        self.install_dir = self.install_dir.canonicalize().unwrap();
        self.backup_dir = self.backup_dir.canonicalize().unwrap();
        self
    }
}
