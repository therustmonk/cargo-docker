# cargo-docker

A cargo subcommand to build Rust code inside docker and get the result back.

It's useful to build binary for other arch. As example, you can build binary for `CentOS` inside `Ubuntu`.

## Usage

To use `cargo-docker` you should type the following:

```sh
$ cargo docker --image=rustup/debian:jessie --output=jessie-target -- --release
```

### Important

This command uses docker without sudo and you have to get rights to use docker.

Add `docker` group:

```sh
$ sudo groupadd docker
$ sudo service docker restart
```

> Not necessary if you have installed new version of docker.

Add yourself to `docker` group:

```sh
$ sudo gpasswd -a ${USER} docker
```

Login again to update user flags.

