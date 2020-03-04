# mandelbrot

`mandelbrot` is a simple Mandelbrot set fractal visualiser for use with our `gWasm` platform.

![mandelbrot GIF demo](https://i.imgur.com/Mlvb3le.gif)

## Building

Before building the program, you'll need to download and install the Emscripten SDK. Follow
the instructions on [Emscripten SDK website] to get the latest copy of the SDK.

Next, install the `wasm32-unknown-emscripten` target. Assuming you're using [rustup] to manage your Rust installation, run the below command:

```
rustup target add wasm32-unknown-emscripten
```

With the WASM target installed you're ready to build the crate:

```
cargo build --release
```

[Emscripten SDK website]: https://emscripten.org/docs/getting_started/downloads.html#installation-instructions
[rustup]: https://rustup.rs

## Running

You'll need a copy of our [gwasm-runner] binary in order to run the program.

Running locally
```
gwasm-runner target/wasm32-unknown-emscripten/release/mandelbrot.wasm -- 1000 1000 2
```

Running on Golem Unlimited
```
gwasm-runner --backend=GU target/wasm32-unknown-emscripten/release/mandelbrot.wasm -- 1000 1000 2
```

Running on Golem network
```
gwasm-runner --backend=Brass target/wasm32-unknown-emscripten/release/mandelbrot.wasm -- 1000 1000 2
```
When running on the Golem network, you may want to adjust some parameters specific to Brass Golem (e.g. the path to your Golem's data directory). To do so, you'll need to create a configuration file for the runner. You can find more information on this in the [gwasm-runner README].

[gwasm-runner]: https://github.com/golemfactory/gwasm-runner/releases
[gwasm-runner README]: https://github.com/golemfactory/gwasm-runner#running-on-the-golem-network
