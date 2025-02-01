# thumbgen

A command-line tool for generating thumbnail images.

> [!WARNING]
> Currently at v0.1.0.
> Only the most basic features are implemented.
> The background image is fixed. You can only set the text displayed on the thumbnail. Since this text is intended for the title and username, the variable names are title and username. It is intended to be used for OGP (Open Graph Protocol) images.

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
