# Installation Docs

## Specific to Linux based platform

- install rust
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- install python
- create virtual env
  - `python3 -m venv .env`
- install pip and requirements
  - `pip3 install -r requirements.txt`
- One time setup: Initialize django project and offline_cash app
  - `django-admin startproject proj`
  - `django-admin startapp offline_cash`
- install diesel cli
  - `cargo install diesel_cli --no-default-features --features "postgres"`

## Database Setup

- Install postgres
- Create login user, so that a person can log in
- use `recreatedb` command to create the service the database
- migrate the migrations in the database using `migrate` command
- create a root user in postresql
- source auto.sh
- run recreate db for migrations


## `zsh` Shell Setup: For Mac

- Install `zsh` and `ohmyzsh`
- Install autoenv plugin to set the environment based on directory
  - https://github.com/Tarrasch/zsh-autoenv


## Run the service

### Run using the cargo

```shell
ENV=demonet cargo run
```


### Build and running the binary 

```shell
cargo build
ENV=demonet ./target/debug/service
```
