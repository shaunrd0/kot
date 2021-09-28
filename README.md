#### kot

Learning to program in Rust by making myself a Linux CLI tool to help manage dotfiles and configurations. 

There are many other tools to manage dotfiles that work just fine. For now, this is intended to be just for my own learning / use and not a general dotfiles management utility.

```bash
[kapper@kubuntu ~]$./kot --help
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
