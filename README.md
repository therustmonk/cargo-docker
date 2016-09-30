# cargo-docker

A cargo subcommand to build Rust code inside docker and get the result back.

It's useful to build binary for other arch. As example, you can build binary for `CentOS` inside `Ubuntu`.

## Usage

To use `cargo-docker` you should type the following:

```sh
$ cargo docker --image=centos:7 -- --release
```
