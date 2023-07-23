# Chip8 Emulator

My first emulator written in Rust.

The result was obtains by following some tutorials :

- https://github.com/aquova/chip8-book
- https://github.com/ColinEberhardt/wasm-rust-chip8
- opcodes : http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

## Build

```shell
cargp build
```

### MacOS build issue

To fix the `ld library not found error` run the following actions :

```sh
 brew install sdl2
 export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Then the build will work.

## Run a demo

```shell
 ./target/debug/desktop ./demo/INVADERS
```