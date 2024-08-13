# <div align="center">Run It</div>

run it is a package manager that is based on containers (yes like apx but declarative and written in rust), the difference is that run it is weighted to be used in both a declarative and an imperative way (the recommended way is the declarative way), run it uses podman in the background for the creation of containers.

## Installing run it ✨

Currently the only way to install run-it is to install it manually.

Requirements:

1.  Podman

Requirements to compile:

1.  Rust
2.  git

then we must run the following commands to compile and have run it installed

```bash
git clone https://github.com/Juanperias/run-it
cd run-it
cargo install runit --path .
```

## Getting Started 🔥

For this we are going to create a container with ubuntu and we are going to see how to do it in a declarative way.

To do this we create a manifest.toml with this content.

```toml
pkgs = ["neofetch"]
container_name = "my-ubuntu"
distro = "ubuntu"
default = true
```

then we do

```bash
runit apply manifest.toml
runit run neofetch
```

and just like that you are using run-it to install neofetch
