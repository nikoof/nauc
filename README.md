# năuc

Năuc (*/nəuk/*) is a simple [brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter and compiler.

By default, the memory is made up of 30000 one-byte cells. The number of cells is configurable via a command line flag.
The cell values are unsigned and wrap by default. Wrapping is configurable.

Other stuff which will be configurable:
- cell size in bits
- cell signed-ness

> [!CAUTION]
> The compiler is currently a work-in-progress. It does no meaningful optimizations. Please report any bugs.


> [!NOTE]
> - The compiler currently supports x86_64-linux and aarch32-linux.
> - On x86_64, it uses the [nasm](https://nasm.us/) assembler as a runtime dependency. On aarch32, it uses the GNU Assembler. Make sure you have your platform's assembler installed.

# Usage
```
Usage: nauc [COMMAND]

Commands:
  interpret  Run in interpreter mode
  compile    Run in compiler mode
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
