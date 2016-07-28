# Pokémon Go Protobuffers for Rust
Exported from and maintained at https://github.com/AeonLucid/POGOProtos

## Usage

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
