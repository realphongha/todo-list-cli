# todo-list-cli
A simple TODO CLI application, written in Rust with SQLite3 as database.

# Installation
## Install from release
Download and unzip the release package.  
Run `install.sh`

## Build from source
```bash
# clone this repo:
git clone https://github.com/realphongha/todo-list-cli.git
# go to the cloned repo:
cd todo-list-cli
# build (you need to have Rust installed first)
cargo build --release
# add `target/release/todo` to your PATH or:
sudo cp target/release/todo /usr/local/bin
```

# Usage
```bash
todo -h
# or
todo --help
```
