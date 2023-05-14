# Git TooLs
[![build](https://github.com/michaelcoll/git-compact/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/michaelcoll/git-compact/actions/workflows/build.yml) ![release](https://badgen.net/github/release/michaelcoll/git-compact?icon=github)

A small tool written in Rust to execute the git gc command in the specified folder and its sub-folders

## Requirements

The `du` and `git` command are needed.

## Usage

```shell
$> gtl --help
Git TooLs

Usage: gtl <COMMAND>

Commands:
  gc    Garbage collect
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```shell
$> gtl gc --help
Garbage collect

Usage: gtl gc [OPTIONS]

Options:
  -f, --folder <FOLDER>  The folder to garbage collect. Git TooLs will search recursively git repository in this folder. By default the current folder is used
  -h, --help             Print help
  -V, --version          Print version
```

This will scan for git repository in your current directory and if it finds one, it will execute the `git gc --aggressive` command.
