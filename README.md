# Pokémon Go Protobuffers for Rust
[![Crates.io](https://img.shields.io/crates/v/pokemon-go-protobuf.svg?maxAge=2592000)](https://crates.io/crates/pokemon-go-protobuf)

Rust library for Pokemon Go proto buffer files, exported from the awesome protos of [https://github.com/AeonLucid/POGOProtos](https://github.com/AeonLucid/POGOProtos)

## Usage

- First of all, add this crate to your dependencies in the `Cargo.toml` file (replace `*` with the version you want):

    ``` toml
    [dependencies]
    pokemon-go-protobuf = "*"
    ```

- Use it!

    ``` rust
    extern crate pokemon_go_protobuf;

    use pokemon_go_protobuf::POGOProtos_Data::PlayerData;

    pub fn main() {
        let x = PlayerData::new();
        // Start using the protobuf structs to encode/decode
    }
    ```

## Update this repository

- Install protobuf 3 and [rust-protobuf](https://github.com/stepancheg/rust-protobuf) (`cargo install protobuf`)

- Update the submodule

    ``` shell
    git submodule init
    git submodule update
    cd POGOProtos
    git pull
    cd ..
    ```

- Execute `./compile.sh`
