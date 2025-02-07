# thumbgen

A command-line tool for generating thumbnail images.

## Installation

```sh
cargo install thumbgen
```

## Usage

Initialize thumbgen. Running this command will generate the `.thumbgen` directory, which contains the `config.toml` file.

```sh
thumbgen init
```

Generate a thumbnail image. The settings in `.thumbgen/config.toml` will be applied.

```sh
thumbgen create
```

## Author

Daiki Nakashima
