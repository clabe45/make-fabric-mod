# Create Fabric Mod

>Scaffolding tool for creating Fabric mods in Java and Kotlin

## Usage

```
$ create-fabric-mod -h
Create a new Fabric mod

Usage: create-fabric-mod [OPTIONS] --id <MOD_ID> --name <NAME> <PATH>

Arguments:
  <PATH>

Options:
  -i, --id <MOD_ID>        Mod ID. Defaults to the name of the directory
  -n, --name <NAME>        Mod name
  -k, --kotlin             Use Kotlin instead of Java
  -m, --main <MAIN_CLASS>  Package and class name of the main class [default: net.fabricmc.example.ExampleMod]
  -h, --help               Print help information
  -V, --version            Print version information
```
