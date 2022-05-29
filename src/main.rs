/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Main entry point for Linux configuration manager kot                ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

use crate::kot::kerror::ErrorKind;

mod kot;

// =============================================================================
// MAIN ENTRY-POINT
// =============================================================================

// -----------------------------------------------------------------------------

fn main() -> kot::Result<()> {
    // Call augmented kot::cli::from_args() to parse CLI arguments
    let mut args = kot::kcli::from_args()?;
    // At this point all paths exist and have been converted to absolute paths
    println!("args: {:?}\n", args);

    // Apply CLI arguments and attempt to install dotfiles
    return kot::handle_args(&mut args);
}
