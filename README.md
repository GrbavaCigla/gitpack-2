# gitpack-2
Better version of C gitpack which is better version of golang gitpack. Package manager....

## Installation

This is the last time you have to clone the repository to install it :)
```
git clone https://github.com/GrbavaCigla/gitpack-2/
cd gitpack-2/
cargo build --release
strip target/release/gitpack
doas/sudo cp target/release/gitpack /usr/bin/
doas/sudo cp res/config.toml /etc/gitpack.toml
```

## Usage
```
gitpack 0.1.0
Gitpack v2, written in rust instead of C, package manager.

USAGE:
    gitpack <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    Install git repository
    list       List all packages
    update     Update all packages
```
Example package install:
```
gitpack install GrbavaCigla/cipherhouse
```
or use url instead:
```
gitpack install -a https://github.com/GrbavaCigla/cipherhouse
```

## Known bugs (also my todo list)
- When installing same package with `-a` and without `-a`, gitpack cannot add package to database and breaks.

## Credits and license
Aleksa Ognjanovic and awesome Rust comunity at discord.  
Licensed under GPLv3.
