#### kot

Learning to program in Rust by making myself a Linux CLI tool to help manage dotfiles and configurations.
There are many other tools to manage dotfiles that work just fine. For now, this is intended to be just for my own learning / use and not a general dotfiles management utility.

Follow [Rustup instructions](https://rustup.rs/) to setup the Rust toolchain

Then to build and run `kot`, run the following commands

```bash
git clone https://gitlab.com/shaunrd0/kot && cd kot
cargo build
./target/debug/kot --help

kot 0.1.0
CLI utility for managing Linux user configurations

USAGE:
    kot [OPTIONS] <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --backup-dir <backup-dir>    The location to store backups for this user [default: backups/kapper]
        --home-dir <install-dir>     The location to attempt installation of user configurations [default: dry-
                                     runs/kapper]

ARGS:
    <config>    Local or full path to user configurations to install
```

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

