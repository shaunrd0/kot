/*##############################################################################
## Author: Shaun Reed                                                         ##
## Legal: All Content (c) 2021 Shaun Reed, all rights reserved                ##
## About: Main entry point for Linux configuration manager kot                ##
##                                                                            ##
## Contact: shaunrd0@gmail.com  | URL: www.shaunreed.com | GitHub: shaunrd0   ##
##############################################################################*/

mod kot;

// =============================================================================
// MAIN ENTRY-POINT
// =============================================================================

// -----------------------------------------------------------------------------

fn main() {
    // Call augmented kot::cli::from_args() to parse CLI arguments
    let args = kot::cli::from_args();
    // At this point all paths exist and have been converted to absolute paths
    println!("args: {:?}\n", args);

    // Attempt to install the configurations, checking for collisions
    match kot::install_configs(&args) {
        Err(e) => {
            // If there was an error, show the error type and run settings
            println!(
                "Error: {:?}\n+ Configs used: {:?}\n+ Install directory: {:?}\n",
                e.kind(), args.dotfiles_dir, args.install_dir
            )
        },
        // Configurations installed successfully
        Ok(()) => (),
    }
}
