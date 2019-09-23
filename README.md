# mandelbrot

`mandelbrot` is a simple Mandelbrot set fractal visualiser for use with our `gWasm` platform.

## Building

Before building the program, you'll need to download and install the Emscripten SDK. Follow
the instructions on [Emscripten SDK website] to get the latest copy of the SDK.

Next, install the `wasm32-unknown-emscripten` target. We assume you're using [rustup] to
manage your Rust installation, so simply run in the terminal/command line

```
rustup target add wasm32-unknown-emscripten
```

Finally, that should be you prepped and ready to build the crate. To do this, simply run
in the terminal/command line

```
cargo build --release
```

[Emscripten SDK website]: https://emscripten.org/docs/getting_started/downloads.html#installation-instructions
[rustup]: https://rustup.rs

## Running

You'll need a copy of our [gwasm-runner] binary in order to run the program

```
gwasm-runner target/wasm32-unknown-emscripten/release/mandelbrot.wasm
```

[gwasm-runner]: https://github.com/golemfactory/gwasm-runner/releases
