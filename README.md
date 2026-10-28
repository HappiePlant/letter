# letter

letter is a package manager for fonts. It defines an openapi spec which repositories can implement.

The CLI is the actual package manager you can run on your system. Currently only Linux is supported.

<!-- TODO: Add windows support -->

Of course this is all written in Rust so it's blazingly fast and memory safe ;)

## Installing the CLI

```sh
cargo install --path crates/letter-cli
```

## Generating the OpenAPI spec

```sh
cargo run --bin letter-generate-openapi --features generate-openapi
```

## Project structure

### `letter` crate

Contains the core models other projects can depend on. Also has a optional feature which uses the `utoipa` crate to generate the OpenAPI specification, which is defined in the `openapi.rs` file. The entrypoint for generating this is in `generate.rs`.

### `letter-cli` crate

Contains the CLI. It uses the `clap` crate as the main "framework" for defining the CLI. The CLI touches fetching from repositories, installing and updating fonts using the metadata defined in repositories, managing installed repositories in a TOML config file and storing installed fonts in a custom store.
