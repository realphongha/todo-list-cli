# todo-list-cli
A simple TODO CLI application, written in Rust with SQLite3 as database.

# Installation
## Quick installation script
```bash
mkdir ~/.todo
cd ~/.todo
wget https://github.com/realphongha/todo-list-cli/releases/download/v0.1.2/todo-list-cli-v0.1.2-x86_64-Linux.zip
tar xvf todo-list-cli-v0.1.2-x86_64-Linux.zip
rm todo-list-cli-v0.1.2-x86_64-Linux.zip
echo 'export PATH="${HOME}/.todo/todo-list-cli-v0.1.2-x86_64-Linux:$PATH"' >> ~/.bashrc
```
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
