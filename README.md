### kot

`kot` is a CLI for managing Linux dotfiles configurations that helps to automate the setup process 
across various applications without risking the loss of local configurations currently on the system.
This helps to protect against installing broken dotfiles by providing a way to reverse the installation 
and return the system back to the previous state.

The installation process creates symbolic links, much like what you would expect when using [stow](https://linux.die.net/man/8/stow).
`kot` can install dotfiles from any source directory, to any target directory. 
To test how `kot` might behave, you could point `--install` to any directory that you've created for testing. 
This directory could be empty, or it could contain another set of dotfiles.
Alternatively, you could set the `--dry-run` flag that will automatically install to a predefined path (`$HOME/.local/share/kot/dry-runs/$USER`)
Note that this directory will never be cleared automatically, each subsequent `--dry-run` 
will stack configurations into this default directory until it is manually cleared.

If conflicts are detected, `kot` shows the conflicts found and 
prompts to abort or continue. An example of this is seen below.
This prompt will be skipped if the `--force` flag is set.

```bash
kot --dry-run dotfiles/dot/

args: Cli { dotfiles: "/home/kapper/Code/kot/dotfiles/dot", install_dir: "/home/kapper/.local/share/kot/dry-runs/kapper", backup_dir: Some("/home/kapper/.local/share/kot/backups/dot:2022-05-29T19:03:27"), clone_dir: None, force: false, dry_run: true, is_repo: false, conflicts: [] }
The following configurations already exist:
  "/home/kapper/.local/share/kot/dry-runs/kapper/.git"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.vimrc"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.bash_aliases"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.vim"
  "/home/kapper/.local/share/kot/dry-runs/kapper/VimScreenshot.png"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.gitignore"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.config"
  "/home/kapper/.local/share/kot/dry-runs/kapper/fix-vbox.sh"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.gitmodules"
  "/home/kapper/.local/share/kot/dry-runs/kapper/.bashrc"
  "/home/kapper/.local/share/kot/dry-runs/kapper/README.md"
If you continue, backups will be made in "/home/kapper/.local/share/kot/backups/dot:2022-05-29T19:03:27". 
Any configurations there will be overwritten.
Continue? Enter Y/y or N/n:


```

#### User Data

`kot` stores user data within `$HOME/.local/share/kot/`

When we provide a repository URL as our `dotfiles` to install, the repo will be *recursively* cloned into 
`$HOME/.local/share/kot/dotfiles/<REPO_NAME>`. 
This is to ensure each user of `kot` maintains their own dotfiles in a location that is accessible but not easy to accidentally modify or erase.
If needed, the user can provide a preferred clone directory to the CLI by setting the `--clone-dir` option

When we encounter conflicts during installation of these dotfiles, backups will be created in
`$HOME/.local/share/kot/backups/<DOTFILES_NAME>:<DATE(%Y-%m-%dT%H:%M:%S)>`
If there are no conflicts found during installation, no backup is created.
Configurations are said to be conflicting if the `--install` path contains configuration files that are
also within the dotfiles we are currently installing.

Backups are intended to reverse changes applied during installation of dotfiles.
These backups are not exhaustive of all configurations tied to the system or user.
The backups only include files that were direct conflicts with configurations being installed.
When we reach an error during installation, `kot` will restore the configurations within the last backup, and then removes unused configurations.

#### Installing kot

Follow [Rustup instructions](https://rustup.rs/) to setup the Rust toolchain

To build and install `kot` run the following commands

```bash
git clone https://gitlab.com/shaunrd0/kot && cd kot
cargo install --path .
kot --help

kot 0.1.5
CLI for managing Linux user configurations

USAGE:
    kot [FLAGS] [OPTIONS] <dotfiles> --install <install>

FLAGS:
    -d, --dry-run
            Installs configurations to $HOME/.local/shared/kot/dry-runs

            Useful flag to set when testing what an install would do to your home directory. This is synonymous with
            setting --install $HOME/.local/shared/kot/dry-runs/$USER. Subsequent runs with this flag set will not delete
            the contents of this directory.
    -f, --force
            Overwrites existing backups

            This flag will replace existing backups if during installation we encounter conflicts and the backup-dir
            provided already contains previous backups.
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -b, --backup-dir <backup-dir>
            The location to store backups for this user

            If no backup-dir is provided, we create one within the default kot data directory:
            $HOME/.local/share/kot/backups/
    -c, --clone-dir <clone-dir>
            An alternate path to clone a dotfiles repository to

            If the clone-dir option is provided to the CLI, kot will clone the dotfiles repository into this directory.
            If clone-dir is not provided, the repository is cloned into $HOME/.local/share/kot/dotfiles Custom clone-dir
            will be used literally, and no subdirectory is created to store the cloned repository For example, clone-dir
            of $HOME/clonedir for repo named Dotfiles We will clone into $HOME/clonedir, and NOT $HOME/clonedir/Dotfiles
            The default path for cloned repos is $HOME/.local/share/kot/dotfiles/
    -i, --install <install>
            The location to attempt installation of user configurations

            The desired installation directory for user configurations. By default this is your $HOME directory This
            could optionally point to some other directory to perform a dry run, or the --dry-run flag could be set
            [env: HOME=/home/kapper]

ARGS:
    <dotfiles>
            Local or full path to user configurations to install. Can also be a git repository.

            System path or repository URL for dotfiles we want to install. If a path is used, it can either be local to
            CWD or absolute. If a URL is used for a dotfiles repository, the repo is cloned into
            $HOME/.local/shared/kot/dotfiles/
```

If you don't want to install `kot`, you can also use the following `cargo` command
 where all arguments after the `--` are passed as arguments to `kot` and not `cargo`.
Below is an example of the short-help output text provided with the `-h` flag
```bash
cd path/to/kot
cargo build
cargo run -- --help


kot 0.1.5
CLI for managing Linux user configurations

USAGE:
    kot [FLAGS] [OPTIONS] <dotfiles> --install <install>

FLAGS:
    -d, --dry-run    Installs configurations to $HOME/.local/shared/kot/dry-runs
    -f, --force      Overwrites existing backups
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --backup-dir <backup-dir>    The location to store backups for this user
    -c, --clone-dir <clone-dir>      An alternate path to clone a dotfiles repository to
    -i, --install <install>          The location to attempt installation of user configurations [env:
                                     HOME=/home/kapper]

ARGS:
    <dotfiles>    Local or full path to user configurations to install. Can also be a git repository
```

#### TODO

* Ensure empty backups are not created
* Provide interface for managing agreed-upon /etc/skel/ configurations
* Provide more CLI options for git functionality; Branches, update submodules, etc
* Clean up warnings during build / installation
* Automate testing
* 
