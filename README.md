# năuc

Năuc (*/nəuk/*) is a simple [brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter.

By default, the memory is made up of 30000 one-byte cells. The number of cells is configurable via a command line flag.
The cell values are unsigned and wrap by default. Wrapping is configurable.

Other stuff which I think should be configurable:
- cell size in bits
- cell signed-ness

In any case, it is currently quite barebones. I am planning to add more flags and eventually make it into a (possibly
optimizing) compiler - *hence the nasm dependency*.

# Usage
```
nauc -h
Usage: nauc [COMMAND]

Commands:
  interpret  Run in interpreter mode
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

# Building
Building is as easy as
```sh
nix build github:nikoof/nauc
```

Alternatively, you can build with Cargo.
```sh
git clone https://github.com/nikoof/nauc && cd nauc
cargo build --release
```

# Contributing
If you somehow decide to use this and find a problem, feel free to open an issue or a PR.
