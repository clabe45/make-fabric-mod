# `make-fabric-mod`

>Scaffolding tool for creating Fabric mods in Java and Kotlin

## Installation

make-fabric-mod can be installed with cargo:

```
$ cargo install make-fabric-mod
```

## Usage

```
$ make-fabric-mod -h
Create a new Fabric mod

Usage: make-fabric-mod [OPTIONS] --name <NAME> --minecraft <MINECRAFT_VERSION> --entrypoint <MAIN_CLASS> <PATH>

Arguments:
  <PATH>  

Options:
  -i, --id <MOD_ID>                    Mod ID. Defaults to the name of the directory [default: ]
  -n, --name <NAME>                    Human-friendly mod name
  -m, --minecraft <MINECRAFT_VERSION>  Minecraft version (x.y)
  -k, --kotlin                         Use Kotlin instead of Java
  -e, --entrypoint <MAIN_CLASS>        Main class (e.g., 'net.fabricmc.example.ExampleMod')
  -h, --help                           Print help information
  -V, --version                        Print version information
```

## Changelog

See the [changelog](CHANGELOG.md) for the up-to-date history of the project's changes.

## License

This project is licensed under [GPL v3](LICENSE).
