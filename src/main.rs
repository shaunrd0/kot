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
    let args = kot::cli::from_args();
    println!("args: {:?}\n", args);

    match kot::install_configs(&args) {
        Err(e) => println!("Error: {:?}\n+ Configs used: {:?}\n+ Install directory: {:?}\n",
                           e.kind(), args.configs_dir, args.install_dir),
        Ok(()) => (),
    }
}
