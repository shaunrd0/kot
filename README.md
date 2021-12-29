### kot

`kot` is a CLI for managing Linux dotfiles configurations that helps to automate the setup process 
across various applications without risking the loss of local configurations currently on the system.
This helps to protect against installing broken dotfiles by providing a way to reverse the installation 
and return the system back to the previous state.

The installation process creates symbolic links, much like what you would expect when using [stow](https://linux.die.net/man/8/stow).
`kot` can install dotfiles from any directory, using any target directory. To test how `kot` might behave, 
you could point `--install-dir` to any directory that you've created for testing. 
This directory could be empty, or it could contain another set of dotfiles. `kot` will attempt
 to install the configurations. If conflicts are detected, output shows the conflicts and 
prompts to abort or continue. An example of this is seen below.

```bash
kot dotfiles/dot/

args: Cli { dotfiles_dir: "/home/kapper/Code/kot/dotfiles/dot", install_dir: "/home/kapper/Code/kot/dry-runs/kapper", backup_dir: "/home/kapper/Code/kot/backups/kapper", force: false }

The following configurations already exist:
  "/home/kapper/Code/kot/dry-runs/kapper/.bashrc"
  "/home/kapper/Code/kot/dry-runs/kapper/.config"
  "/home/kapper/Code/kot/dry-runs/kapper/README.md"
  "/home/kapper/Code/kot/dry-runs/kapper/VimScreenshot.png"
  "/home/kapper/Code/kot/dry-runs/kapper/fix-vbox.sh"
  "/home/kapper/Code/kot/dry-runs/kapper/.git"
  "/home/kapper/Code/kot/dry-runs/kapper/.bash_aliases"
  "/home/kapper/Code/kot/dry-runs/kapper/.gitignore"
  "/home/kapper/Code/kot/dry-runs/kapper/.gitmodules"
  "/home/kapper/Code/kot/dry-runs/kapper/.vimrc"
  "/home/kapper/Code/kot/dry-runs/kapper/.vim"
If you continue, backups will be made in "/home/kapper/Code/kot/backups/kapper". Any configurations there will be overwritten.
Abort? Enter y/n or Y/N:
```

If there are already files within the backup directory, `kot` will exit and show an error message.
This is to protect existing backups from being merged with configs from subsequent runs.
If you want to erase these backups and create a new backup, rerun the command with the `--force` flag set.
Otherwise, specify a different backup directory with the `--backup-dir` option. 
If the backup directory does not exist, it will be created.


```bash
kot dotfiles/dot/

thread 'main' panicked at '
  Error: Backups already exist at "/home/kapper/Code/kot/backups/kapper"
  Set the --force flag to overwrite configurations stored here', src/kot/kcli.rs:94:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Installation

Follow [Rustup instructions](https://rustup.rs/) to setup the Rust toolchain

To build and install `kot` run the following commands

```bash
git clone https://gitlab.com/shaunrd0/kot && cd kot
cargo install --path .
kot --help

kot 0.1.0
CLI for managing Linux user configurations

USAGE:
    kot [FLAGS] [OPTIONS] <dotfiles-dir>

FLAGS:
    -f, --force      Overwrites existing backups
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --backup-dir <backup-dir>      The location to store backups for this user [default: backups/kapper]
    -i, --install-dir <install-dir>    The location to attempt installation of user configurations [default: dry-
                                       runs/kapper]

ARGS:
    <dotfiles-dir>    Local or full path to user configurations to install
```

#### Dotfiles Management

To store dotfiles, this repository uses submodules. To update surface-level submodules, we can run the following commands
```bash
git submodule update --init

Submodule path 'dot': checked out '7877117d5bd413ecf35c86efb4514742d8136843'
```

But in the case of my dotfiles repository, [shaunrd0/dot](https://gitlab.com/shaunrd0/dot), I use submodules to clone repositories for vim plugins. To update all submodules *and their nested submodules*, we can run the following commands
```bash
git submodule update --init --recursive

Submodule 'dotfiles/dot' (https://gitlab.com/shaunrd0/dot) registered for path 'dotfiles/dot'
Cloning into '/home/kapper/Code/kotd/dotfiles/dot'...
warning: redirecting to https://gitlab.com/shaunrd0/dot.git/
Submodule path 'dotfiles/dot': checked out '7877117d5bd413ecf35c86efb4514742d8136843'
Submodule '.vim/bundle/Colorizer' (https://github.com/chrisbra/Colorizer) registered for path 'dotfiles/dot/.vim/bundle/Colorizer'
Submodule '.vim/bundle/ale' (https://github.com/dense-analysis/ale) registered for path 'dotfiles/dot/.vim/bundle/ale'
Submodule '.vim/bundle/clang_complete' (https://github.com/xavierd/clang_complete) registered for path 'dotfiles/dot/.vim/bundle/clang_complete'
Submodule '.vim/bundle/supertab' (https://github.com/ervandew/supertab) registered for path 'dotfiles/dot/.vim/bundle/supertab'
Submodule '.vim/bundle/unicode.vim' (https://github.com/chrisbra/unicode.vim) registered for path 'dotfiles/dot/.vim/bundle/unicode.vim'
Submodule '.vim/bundle/vim-airline' (https://github.com/vim-airline/vim-airline) registered for path 'dotfiles/dot/.vim/bundle/vim-airline'
Submodule '.vim/bundle/vim-airline-themes' (https://github.com/vim-airline/vim-airline-themes) registered for path 'dotfiles/dot/.vim/bundle/vim-airline-themes'
Submodule '.vim/bundle/vim-signify' (https://github.com/mhinz/vim-signify) registered for path 'dotfiles/dot/.vim/bundle/vim-signify'
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/Colorizer'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/ale'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/clang_complete'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/supertab'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/unicode.vim'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/vim-airline'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/vim-airline-themes'...
Cloning into '/home/kapper/Code/kotd/dotfiles/dot/.vim/bundle/vim-signify'...
Submodule path 'dotfiles/dot/.vim/bundle/Colorizer': checked out '826d5691ac7d36589591314621047b1b9d89ed34'
Submodule path 'dotfiles/dot/.vim/bundle/ale': checked out '3ea887d2f4d43dd55d81213517344226f6399ed6'
Submodule path 'dotfiles/dot/.vim/bundle/clang_complete': checked out '293a1062274a06be61797612034bd8d87851406e'
Submodule path 'dotfiles/dot/.vim/bundle/supertab': checked out 'd80e8e2c1fa08607fa34c0ca5f1b66d8a906c5ef'
Submodule path 'dotfiles/dot/.vim/bundle/unicode.vim': checked out 'afb8db4f81580771c39967e89bc5772e72b9018e'
Submodule path 'dotfiles/dot/.vim/bundle/vim-airline': checked out 'cb1bc19064d3762e4e08103afb37a246b797d902'
Submodule path 'dotfiles/dot/.vim/bundle/vim-airline-themes': checked out 'd148d42d9caf331ff08b6cae683d5b210003cde7'
Submodule path 'dotfiles/dot/.vim/bundle/vim-signify': checked out 'b2a0450e23c63b75bbeabf4f0c28f9b4b2480689'
```
