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

Usage: make-fabric-mod [OPTIONS] --name <NAME> <PATH>

Arguments:
  <PATH>

Options:
  -i, --id <MOD_ID>        Mod ID. Defaults to the name of the directory [default: ]
  -n, --name <NAME>        Mod name
  -k, --kotlin             Use Kotlin instead of Java
  -m, --main <MAIN_CLASS>  Main class [default: net.fabricmc.example.ExampleMod]
  -h, --help               Print help information
  -V, --version            Print version information
```
